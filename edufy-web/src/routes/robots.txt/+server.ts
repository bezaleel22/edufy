import { SITE_URL } from '$lib/constants';

export async function GET() {
  const robots = `User-agent: *
Allow: /

# Sitemap
Sitemap: ${SITE_URL}/sitemap.xml

# Crawl-delay
Crawl-delay: 1

# Disallow admin areas (if any)
Disallow: /admin/
Disallow: /_app/
Disallow: /api/

# Allow important pages
Allow: /
Allow: /about
Allow: /admission
Allow: /portfolio
Allow: /blog
Allow: /contact`;

  return new Response(robots, {
    headers: {
      'Content-Type': 'text/plain',
      'Cache-Control': 'max-age=86400'
    }
  });
}
