import type { Handle } from '@sveltejs/kit';
import { sequence } from '@sveltejs/kit/hooks';

// Simplified cache control hook - ONLY for public blog content
const cacheControl: Handle = async ({ event, resolve }) => {
  const response = await resolve(event);
  
  // Set cache headers ONLY for public blog pages
  if (event.url.pathname.startsWith('/blog') && !response.headers.get('Cache-Control')) {
    // All blog content cached at edge (private posts handled by API directly)
    response.headers.set('Cache-Control', 'public, max-age=60, s-maxage=3600');
  }
  
  // Set security headers as per CMS.md requirements
  response.headers.set('X-Content-Type-Options', 'nosniff');
  response.headers.set('X-Frame-Options', 'DENY');
  response.headers.set('X-XSS-Protection', '1; mode=block');
  response.headers.set('Referrer-Policy', 'strict-origin-when-cross-origin');
  
  return response;
};

// NO authentication or authorization hooks - private areas use CSR + direct API auth
export const handle = cacheControl;
