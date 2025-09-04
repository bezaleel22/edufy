# Deployment Guide - LLA Web Architecture

This document explains how the LLA Web project has been refactored to conform to the architecture specified in CMS.md.

## Architecture Overview

The project follows a clean separation between:

1. **Frontend**: SvelteKit app deployed to Cloudflare Pages (SSR)
2. **Backend/CMS**: Rust app serving APIs at `school.llacademy.ng`
3. **Storage**: Cloudflare KV, D1, Images, and R2

## Components Alignment

### 1. SvelteKit Frontend (Cloudflare Pages)

**Files Updated:**
- `svelte.config.js` - Configured for Cloudflare adapter with proper SSR settings
- `src/hooks.server.ts` - Implements authentication, caching, and authorization hooks
- `src/app.d.ts` - TypeScript definitions for Cloudflare bindings
- `wrangler.toml` - Cloudflare Pages configuration with KV, D1, R2 bindings
- `package.json` - Updated dependencies and scripts

**Key Features:**
- SSR for blog routes (`/blog/*`) with KV access
- SSR for portal routes (`/portal/*`) with D1 authentication
- Prerendered static pages (home, about, etc.)
- Proper cache headers (`Cache-Control: public, max-age=60, s-maxage=3600`)
- Secure cookie handling with environment-aware settings

### 2. Rust CMS Backend

**Files Updated:**
- `cms/src/main.rs` - Environment-aware KV storage initialization
- `cms/src/handlers.rs` - CORS configuration for cross-domain requests
- `cms/src/auth.rs` - Environment-aware cookie configuration
- `cms/src/kv.rs` - KV abstraction supporting both local dev and Cloudflare KV API

**Key Features:**
- Admin API endpoints with authentication middleware
- Public blog endpoints for SvelteKit SSR
- Secure HttpOnly cookies with proper domain settings
- Cloudflare KV integration for blog content
- D1 database for authentication and audit logs

### 3. Data Storage Strategy

**KV (BLOG_KV):**
- `blog:post:{slug}` - Individual blog posts
- `blog:index` - Sorted list of blog entries

**D1 (school_auth):**
- `users` - User authentication
- `sessions` - Session management
- `audit_logs` - Audit trail

**R2 (school-media):**
- File storage for media assets

## Environment Configuration

### Development
- Local SQLite database
- Local file-based KV storage
- HTTP cookies with `SameSite=Lax`
- CMS API at `localhost:8080`

### Production
- Cloudflare KV API integration
- Secure HTTPS cookies with `Domain=.llacademy.ng`
- `SameSite=None; Secure; HttpOnly`
- CMS API at `school.llacademy.ng`

## Security Implementation

1. **Authentication**: JWT tokens stored in secure HttpOnly cookies
2. **CORS**: Configured for `llacademy.ng` and `www.llacademy.ng`
3. **Caching**: Private content uses `Cache-Control: private, no-store`
4. **Headers**: Security headers implemented in hooks

## Deployment Steps

### Frontend (Cloudflare Pages)
1. Connect GitHub repo to Cloudflare Pages
2. Set build command: `npm run build`
3. Configure environment variables in Cloudflare dashboard
4. Set up KV, D1, and R2 bindings

### Backend (Rust CMS)
1. Build using Dockerfile and publish to github container registry and Deploy to VPS using Dokploy
2. Set environment variables for production
3. Configure Cloudflare proxy for `school.llacademy.ng`
4. Set up database migrations

## Monitoring & Observability

- Structured JSON logging in Rust CMS
- Error tracking in SvelteKit hooks
- Health check endpoint at `/healthz`
- Audit logging for all admin actions

## Cache Strategy

- **Public blog posts**: `Cache-Control: public, max-age=60, s-maxage=3600`
- **Private content**: `Cache-Control: private, no-store`
- **Static pages**: Prerendered and cached at edge
- **KV reads**: Fast edge access with fallback to API

This architecture ensures optimal performance, security, and scalability while maintaining clear separation of concerns between frontend and backend components.
