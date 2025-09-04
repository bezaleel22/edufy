import { redirect } from '@sveltejs/kit';

export const load = async ({ url, fetch }: any) => {
  // Get redirect URL from query params
  const redirectUrl = url.searchParams.get('redirect') || '/portal';
  
  return {
    redirectUrl
  };
};

export const ssr = false; // Force client-side rendering
export const prerender = false; // No prerendering for auth pages
