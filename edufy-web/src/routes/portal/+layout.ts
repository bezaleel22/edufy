import { redirect } from '@sveltejs/kit';
import { browser } from '$app/environment';

export const load = async ({ url, fetch }: any) => {
  if (browser) {
    // Check authentication on client side
    try {
      const response = await fetch('/api/users/me', {
        credentials: 'include'
      });
      
      if (!response.ok) {
        // Not authenticated, redirect to login
        throw redirect(302, `/login?redirect=${encodeURIComponent(url.pathname)}`);
      }
      
      const user = await response.json();
      return { user };
    } catch (error) {
      if (error instanceof Response) {
        throw error;
      }
      // Network error or other issue
      throw redirect(302, `/login?redirect=${encodeURIComponent(url.pathname)}`);
    }
  }
  
  return {};
};

export const ssr = false; // Force client-side rendering
export const prerender = false; // No prerendering for auth pages
