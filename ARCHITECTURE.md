# Architecture Overview — components (Updated 2025)

* **Frontend**: SvelteKit app deployed to **Cloudflare Pages (Hybrid SSR + CSR)**
* **Backend / CMS**: Rust app (Dokploy on VPS) serving APIs at `portal.llacademy.ng`
* **Storage & Edge services (Cloudflare)**: KV (`BLOG_KV`), Cloudflare Images, R2 (`school-media`)
* **Database**: **Local SQLite with WAL mode** (migrated from Cloudflare D1 for performance and cost optimization)
* **Domains**:

  * `llacademy.ng`, `www.llacademy.ng` → SvelteKit (Pages)
  * `portal.llacademy.ng` → Rust API (proxied by Cloudflare)
  * `media.llacademy.ng` → Images & R2 CDN (fronted by Cloudflare)
* **Secrets & Config**: Cloudflare environment variables (via API or dashboard), Vault for CI, GitHub Actions secrets (or similar)

## Performance Optimization (Updated)
* **Authentication Strategy**: **JWT with revocation list** for performance (no session table overhead)
* **Database**: **SQLite in WAL mode** on NVMe for optimal read/write performance
* **Private Areas**: CSR with direct API authentication for optimal performance
* **Public Content**: SSR with edge caching for SEO and performance
* **Audit Logging**: **Incremental JSON appends** with monthly sharding for compliance

---

# 1. High-level pipeline (end-to-end)

1. **Admin login & content creation**

   * Teacher logs into Rust CMS (**Google OAuth passwordless authentication**).
   * Upload assets: Images → Cloudflare Images; Big files → R2.
   * CMS composes final post JSON (body\_html, metadata, asset URLs, visibility).
   * CMS writes post to KV (`blog:post:{slug}`) and updates `blog:index`.
   * CMS writes audit entry into **local SQLite with incremental JSON logging**.

2. **Public site rendering**

   * User hits `llacademy.ng/blog` or `/blog/{slug}`.
   * SvelteKit SSR (Pages function) reads `blog:index` or `blog:post:{slug}` from KV.
   * Private posts are filtered out at the API level - no server-side session verification in SvelteKit.
   * SvelteKit renders page, referencing asset URLs on `media.llacademy.ng`.

3. **Private area access (CSR approach)**

   * User accesses `/portal/*` or `/admin/*` routes.
   * Client-side authentication check using localStorage + API verification.
   * Direct API calls to `portal.llacademy.ng` with **JWT tokens** via `credentials: 'include'`.
   * **JWT revocation list** stored in SQLite for security.
   * No server-side authentication in SvelteKit hooks - optimized for performance.

4. **Preview / Drafts**

   * CMS pushes draft posts to a `drafts` KV namespace.
   * Preview URLs include signed tokens (short TTL) that SvelteKit verifies server-side.

5. **Backup & rotation**

   * **Daily SQLite backups** to **SharePoint Drive** via MS Graph API.
   * **Monthly audit log archival** with table sharding.
   * CMS pushes rotating webhook secrets or keys into Cloudflare env via API.

---

# 2. SvelteKit-side: Implementation details & best practices

## Structure & routing

* Routes: `edufy-web/src/routes`

  * `/` (prerendered)
  * `/about`, `/admissions`, `/contact`, `/academics`, `/portfolio`, `/robots.txt`, `/sitemap.xml` (prerendered)
  * `/blog` (SSR; reads `blog:index` from KV)
  * `/blog/[slug]` (SSR; reads `blog:post:{slug}` from KV; server-only logic)
  * `/portal/*` (CSR; requires authenticated session checked against **local SQLite/Rust API**)

## svelte.config.js

* Use `@sveltejs/adapter-cloudflare` with `routes.exclude: ['/blog/*']` if you want to force SSR for blog, but in this design SvelteKit will SSR blog pages reading KV.
* Bindings configured for `BLOG_KV` and any env secrets.

## Server-side fetching & caching pattern

* `/blog` load():

  * `const index = await platform.env.BLOG_KV.get('blog:index');`
  * Parse and render list.
  * Set cache-control headers on the SSR response: `public, max-age=60, s-maxage=3600` (short local, long edge).
* `/blog/[slug]` load():

  * `const post = await BLOG_KV.get('blog:post:{slug}');`
  * **Private posts are filtered at the API level** - no server-side session verification in SvelteKit for performance.
  * Return `post` into page and render `{@html post.body_html}`.

## Hook: handle() / global cache control (Updated for Performance)

* Implement simplified `src/hooks.server.ts` handle to:

  * **NO authentication parsing** - to avoid Cloudflare execution time limits.
  * **Only** set Cache-Control headers for public blog pages.
  * **Security headers** for all requests (X-Content-Type-Options, X-Frame-Options, etc.).
  * **Private areas use CSR** - authentication handled client-side with direct API calls.

## Caching strategy (edge + origin)

* KV reads are fast; still set `Cache-Control`:

  * Public posts: `Cache-Control: public, max-age=60, s-maxage=3600`
  * Private posts: `Cache-Control: private, no-store`
* Let Cloudflare's CDN serve cached HTML for anonymous users. Use `stale-while-revalidate` patterns if needed.

## Authentication (frontend)

* **Hybrid authentication strategy for performance optimization:**
  * **Public content**: No authentication required in SvelteKit - optimized for speed
  * **Private areas**: CSR authentication using localStorage + API verification
* Use **JWT tokens** set by Rust API for portal sessions.
* For client-side API calls (from browser to `portal.llacademy.ng`) use `fetch(..., { credentials: 'include' })`.
* Authentication state managed in client-side layouts (e.g., `/portal/admin/+layout.svelte`).

## Preview mode & drafts

* CMS writes drafts to `KV` with key `draft:blog:post:{slug}:{preview_token}`.
* Preview routes use CSR authentication pattern for performance - token validation done via API calls to Rust backend.

## Error handling

* If KV read fails: return 503 with friendly maintenance page.
* If post not found: 404 template.
* Log errors (discussed in observability).

---

# 3. Rust CMS-side: Implementation details & best practices 
* Directory: `src/main.rs`

## Core responsibilities

* Admin UI & API (auth, uploads, post management)
* **Google OAuth passwordless authentication**
* **JWT token management with revocation list**
* Image uploads → Cloudflare Images API
* File uploads → R2 (signed PUT URL or server-side PUT)
* KV writes for posts & index
* **SQLite writes for users, JWT revocations, incremental audit logs**
* **Daily backups to SharePoint Drive**

## API design (REST endpoints)

### Authentication
* `POST /api/auth/login` → returns **JWT token** (passwordless login)
* `POST /api/auth/google` → **Google OAuth login** → returns JWT token
* `POST /api/auth/logout` → **adds JWT to revocation list**
* `GET /api/users/me` → verify **JWT token** (checks revocation list)

### Admin Blog Management
* `GET /api/admin/posts` → admin-only list (includes private posts)
* `POST /api/admin/posts` → create post: uploads images/R2 → write KV
* `GET /api/admin/posts/{slug}` → get post (admin view)
* `PUT /api/admin/posts/{slug}` → update post: re-upload assets, update KV
* `DELETE /api/admin/posts/{slug}` → delete KV key + remove assets
* `POST /api/admin/posts/model` → **create post using BlogPost model**
* `PUT /api/admin/posts/model/{slug}` → **update post using BlogPost model**

### Public Blog API (for SvelteKit SSR)
* `GET /api/blog/index` → public posts index for SvelteKit
* `GET /api/blog/post/{slug}` → get public post
* `GET /api/blog/public/{slug}` → **direct public post access**

### Admin User Management
* `GET /api/admin/users/{user_id}` → **get user by ID**
* `GET /api/admin/users/email/{email}` → **get user by email**
* `GET /api/admin/users/{user_id}/role/{role}` → **check user role**

### Admin Audit & Backup
* `GET /api/admin/audit/logs/{user_id}` → **get user audit logs**
* `POST /api/admin/audit/cleanup` → **cleanup old audit tables** (monthly sharding)
* `POST /api/admin/backup/restore` → **restore database from SharePoint backup**

### Admin Media Upload
* `POST /api/admin/upload/image` → **upload image via JSON/base64**
* `POST /api/admin/upload/file` → **upload file via JSON/base64**
* `POST /api/admin/upload/multipart` → **upload files via multipart form**

## Auth & sessions (SQLite with WAL mode)

* **Passwordless authentication**: Google OAuth only, no local passwords.
* **JWT tokens**: issued on successful authentication with expiration.
* **Revocation list**: stored in SQLite `revocations` table with JTI (JWT ID).
* **Token security**: JWT includes user ID, role, expiration, and unique JTI.
* **Performance**: No session table overhead - JWT validation with revocation check.

### Database Schema (SQLite)
```sql
-- Users (no password_hash - passwordless)
users (
  id TEXT PRIMARY KEY,
  email TEXT UNIQUE NOT NULL,
  role TEXT CHECK(role IN ('student','parent','teacher','admin')),
  google_id TEXT UNIQUE,
  full_name TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)

-- JWT Revocation List
revocations (
  jti TEXT PRIMARY KEY,
  user_id TEXT,
  revoked_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  expires_at TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES users(id)
)

-- Monthly Sharded Audit Logs (e.g., audit_logs_2025_09)
audit_logs_YYYY_MM (
  id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL,
  session_date TEXT NOT NULL, -- YYYY-MM-DD
  actions TEXT NOT NULL DEFAULT '[]', -- JSON array
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)
```

## Uploading images & files

* **Images**: upload to Cloudflare Images via API, record returned image id/URL in KV post JSON.
* **R2**: store large assets using authenticated PUTs; prefer uploading from CMS backend to R2 and return CDN URL.
* **MediaUploader**: centralized upload handling with environment-aware configuration (local dev vs production).

## Publishing flow inside CMS

1. Upload images/files → get URLs
2. Render or sanitize post HTML
3. Compose JSON: `{id, title, slug, body_html, tags, author_id, date_published, visibility, cover_image, attachments}`
4. PUT to KV: `blog:post:{slug}` and update `blog:index` (atomic-like: write `blog:post:{slug}` then `blog:index`).
5. Write audit log into **SQLite with incremental JSON append**.

## Safety: sanitization

* Always sanitize `body_html` on server (strip unsafe tags, attributes) before saving to KV.
* Consider storing original Markdown and rendering to HTML in CMS controlled renderer.

---

# 4. Data models (detailed)

## KV (BLOG\_KV)

* Key: `blog:post:{slug}`

  * Value JSON:

    * `id` (uuid)
    * `title` (string)
    * `slug` (string)
    * `summary` (string)
    * `body_html` (string)
    * `author_id` (uuid)
    * `tags` (array)
    * `date_published` (ISO8601)
    * `visibility` (`public`|`private`)
    * `cover_image` (image id or URL)
    * `attachments` (array of R2 URLs)
    * `meta` (optional extra metadata)
* Key: `blog:index` — JSON array of `{slug, title, summary, cover_image, date_published, tags}` sorted desc

## SQLite schema (WAL mode optimized)

### Performance Optimizations
* **WAL mode**: `PRAGMA journal_mode = WAL;`
* **Optimized sync**: `PRAGMA synchronous = NORMAL;`
* **Page cache**: `PRAGMA cache_size = -65536;` (64MB)
* **Memory temp**: `PRAGMA temp_store = MEMORY;`

### Tables
* `users`: id, email, role, google_id, full_name, created_at
* `revocations`: jti, user_id, revoked_at, expires_at
* `audit_logs_YYYY_MM`: id, user_id, session_date, actions (JSON), created_at, updated_at

### Indexes
* `idx_users_email`, `idx_users_google_id`
* `idx_revocations_jti`, `idx_revocations_user_id`
* `idx_audit_logs_YYYY_MM_user_id`, `idx_audit_logs_YYYY_MM_session_date`

---

# 5. Naming conventions (concise)

* Slugs: `lowercase-words-separated-by-dashes`
* KV keys:

  * Posts: `blog:post:{slug}`
  * Index: `blog:index`
  * Drafts: `blog:draft:{slug}:{token}`
* Images (Cloudflare Images): `blog-{slug}-cover`, `blog-{slug}-img-{n}` (map ID to KV)
* R2 objects: `reports/{year}/{slug}.pdf`, `events/{year}/{slug}.mp4`
* SQLite GUIDs: lowercase UUID v4
* Timestamps: ISO 8601 UTC (`2025-09-01T12:00:00Z`)
* **Audit tables**: `audit_logs_YYYY_MM` (monthly sharding)

---

# 6. Security & networking

## Authentication & CORS

* **JWT tokens**: issued by Rust API, validated against revocation list
* **Google OAuth**: primary authentication method (passwordless)
* API: `Access-Control-Allow-Origin: https://llacademy.ng` (not `*`) and `Access-Control-Allow-Credentials: true`
* Cloudflare rules: bypass caching for `/api/*` (Cache Rule: `portal.llacademy.ng/api/*` → Cache Level: Bypass)
* TLS: Cloudflare SSL settings: **Full (Strict)**; keep Dokploy TLS cert valid.

## Cloudflare firewall & DDoS

* Apply WAF rules to `portal.llacademy.ng` (block known bad IPs/SQLi or XSS patterns).
* Rate limit auth endpoints `POST /api/auth/*` (e.g., 5 req/min per IP).
* Create IP allow/deny lists for admin endpoints if desired.

## Secrets

* Store Cloudflare API tokens in a secure vault; for automation use short-lived tokens if possible.
* CMS must have an API token with minimal scope: only KV write + Images/R2 write as required.
* **Google OAuth secrets**: client ID, client secret, redirect URI configured in environment.
* **SharePoint secrets**: tenant ID, client ID, client secret for backup integration.
* Rotate secrets periodically and provide rotation endpoint in CMS to push new env to Cloudflare.

---

# 7. Observability, logging & backups

## Logging & metrics

* Rust CMS logs: structured JSON logs (timestamp, level, request\_id, user\_id, route, duration).
* SvelteKit errors: capture stack traces server-side, forward critical errors to a monitoring system.
* Use a log aggregator (e.g., Logflare / remote ELK / Datadog) for retention & search.
* Instrument key metrics: KV read latency, page response time, **SQLite read/write counts**, Images/R2 bandwidth usage.

## Health checks

* `/healthz` on Rust API returning **SQLite connectivity** and disk space; configure Cloudflare uptime checks or external monitors.

## Backups

* **SQLite**: **daily automated backups to SharePoint Drive** via MS Graph API.
* **Backup retention**: 30 days in SharePoint, optional monthly archive to R2.
* **Audit log archival**: monthly cleanup of old audit tables, export to SharePoint.
* R2: lifecycle backup or replicate to another storage (optional).
* KV: export keys or use your CMS to keep authoritative copy (posts can be re-sourced from CMS).

---

# 8. Failure modes & mitigations

| Failure                       |                             Likely cause | Mitigation                                                                           |
| ----------------------------- | ---------------------------------------: | ------------------------------------------------------------------------------------ |
| KV read fails                 | Cloudflare incident or binding misconfig | Return cached static page fallback; show maintenance message; alert ops              |
| KV eventual consistency delay |                  Write propagation delay | CMS can wait 1–2s before confirming publish; show "publishing" state                 |
| **SQLite corruption**         |                   **Disk/NVMe failure** | **Daily SharePoint backups; WAL mode reduces risk; restore from latest backup**     |
| **JWT revocation list growth** |               **Long-lived tokens** | **Cleanup expired revocations; monitor revocation table size**                      |
| **Audit table growth**        |              **High user activity** | **Monthly sharding; automated cleanup; SharePoint archival**                        |
| Image upload failure          |                           Network or API | Retry with backoff; keep local copy and retry; notify admin                          |
| Exceeded KV writes (1k/day)   |                  Bulk content publishing | Throttle publishes; batch index updates; use staging -> bulk write in off-peak hours |
| Token compromise              |                             API key leak | Rotate tokens; revoke old ones; add IP restrictions to tokens                        |
| **SharePoint backup failure** |         **MS Graph API issues** | **Monitor backup status; fallback to local retention; alert on failures**          |

---

# 9. Scalability & cost-control tips

* Cache aggressively at edge for public pages.
* Use `s-maxage` / `stale-while-revalidate` to trade latency for freshness.
* Avoid SSR where static pre-render works.
* Use Cloudflare Images to reduce bandwidth & transformation costs.
* **SQLite on NVMe**: handles tens of thousands of DAU efficiently.
* **JWT with revocation**: eliminates session table overhead.
* **Incremental audit logging**: reduces write amplification vs row-per-action.
* Monitor usage (KV reads/writes, **SQLite size**, R2 egress) and set budget alerts.

---

# 10. CI/CD & deployment workflow

## SvelteKit (Pages)

* GitHub repo → Cloudflare Pages (connect via repo)
* Build command: `npm run build`
* Build output: automatically configured for Cloudflare adapter (or `.svelte-kit/cloudflare`)
* Use preview branches sparingly (free tier build limits). Batch updates in one PR when possible.

## Rust CMS

* GitHub Actions → github container registry + deploy to VPS via Dokploy
* Keep release tags; perform **SQLite migrations** with migration tool (run migrations in CI/CD step)
* During deploy: warm KV or pre-populate frequently-read content if needed.
* **Database migrations**: automated via `migrations/*.sql` files.

## Secrets in CI

* Store Cloudflare API tokens and **SQLite** credentials in GitHub Secrets.
* **Google OAuth secrets** in secure environment variables.
* **SharePoint secrets** for backup integration.
* Use least-privilege tokens; rotate regularly.

---

# 11. Operational runbook (short)

* **Publish failure**: check CMS logs → KV PUT status. Retry writes and notify staff.
* **KV read 404 for existing post**: check `blog:post:{slug}` via `wrangler` or Cloudflare API. Check propagation delay.
* **Image broken**: verify R2/Images URL; confirm CDN mapping `media.llacademy.ng` is set and cert valid.
* **Auth issues**: check **SQLite connectivity**, **JWT revocation list**, and token validation.
* **High latency**: identify origin vs CDN; check KV latency; check **SQLite query performance**.
* **Backup failure**: check **SharePoint connectivity**; verify MS Graph API credentials; manual backup if needed.
* **Audit table growth**: run monthly cleanup; check **audit_logs_YYYY_MM** table sizes.

---

# 12. Appendix — Quick reference snippets (updated)

* KV keys: `blog:post:back-to-school-2025`, `blog:index`, `blog:draft:...`
* **SQLite tables**: `users`, `revocations`, `audit_logs_2025_09`
* Images names: `blog-back-to-school-2025-cover`
* R2 paths: `reports/2025/back-to-school.pdf`
* **JWT structure**: `{user_id, role, exp, jti}` with HS256 signing
* **Google OAuth flow**: `POST /api/auth/google` with OAuth code → JWT token
* **SharePoint backup**: Daily automated via MS Graph API to configured drive

## Environment Variables (Updated)

### Core Configuration
* `DATABASE_URL` → SQLite database path (default: `sqlite:cms.db`)
* `JWT_SECRET` → JWT signing secret
* `PORT` → Server port (default: 3001)

### Google OAuth
* `GOOGLE_CLIENT_ID` → Google OAuth client ID
* `GOOGLE_CLIENT_SECRET` → Google OAuth client secret
* `GOOGLE_REDIRECT_URI` → OAuth callback URL

### Cloudflare Integration
* `CLOUDFLARE_ACCOUNT_ID` → Cloudflare account ID
* `CLOUDFLARE_API_TOKEN` → Cloudflare API token (KV + Images + R2)
* `MEDIA_DOMAIN` → CDN domain for media assets

### SharePoint Backup
* `SHAREPOINT_TENANT_ID` → Microsoft tenant ID
* `SHAREPOINT_CLIENT_ID` → SharePoint app client ID
* `SHAREPOINT_CLIENT_SECRET` → SharePoint app secret
* `SHAREPOINT_SITE_ID` → Target SharePoint site
* `SHAREPOINT_DRIVE_ID` → Target drive for backups

### Backup Configuration
* `BACKUP_ENABLED` → Enable/disable automated backups
* `BACKUP_SCHEDULE` → Cron expression for backup timing
* `BACKUP_RETENTION_DAYS` → Days to retain backups
