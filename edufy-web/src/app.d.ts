// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			user?: {
				id: string;
				email: string;
				role: string;
				full_name: string;
			};
			token?: string;
		}
		// interface PageData {}
		interface PageState {}
		interface Platform {
			env: {
				// Cloudflare KV binding for blog content
				BLOG_KV: KVNamespace;
				// Cloudflare D1 binding for authentication and audit logs
				D1: D1Database;
				// Cloudflare R2 binding for media storage
				R2_BUCKET: R2Bucket;
				// Environment variables
				CMS_API_URL: string;
				ENVIRONMENT: string;
				DOMAIN: string;
				// Cloudflare API credentials (for CMS)
				CLOUDFLARE_API_TOKEN?: string;
				CLOUDFLARE_ACCOUNT_ID?: string;
				CLOUDFLARE_KV_NAMESPACE_ID?: string;
			};
			context: {
				waitUntil(promise: Promise<any>): void;
			};
			caches: CacheStorage & { default: Cache };
		}
	}
}

export {};
