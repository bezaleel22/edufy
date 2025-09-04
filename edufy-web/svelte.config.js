import adapter from "@sveltejs/adapter-cloudflare";
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: [vitePreprocess()],
	kit: {
		adapter: adapter({
			// Configure for SSR with Cloudflare bindings (KV, D1, R2)
			routes: {
				include: ['/*'],
				exclude: [
					'/robots.txt',
					'/sitemap.xml'
					// Blog routes use SSR for KV access
					// Portal routes use SSR for authentication
				]
			}
		}),
		// Configure prerendering according to CMS.md architecture
		prerender: {
			entries: [
				'/',
				'/about',
				'/admission',
				'/contact',
				'/academics',
				'/portfolio',
				'/robots.txt',
				'/sitemap.xml'
			],
			crawl: false
		},
		// Set up environment variables for platform bindings
		env: {
			privatePrefix: 'PRIVATE_',
			publicPrefix: 'PUBLIC_'
		}
	},
};

export default config;
