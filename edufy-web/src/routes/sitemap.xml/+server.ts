import { SITE_URL } from "$lib/constants";
import { blogManager } from "$lib/data/blogManager";

export async function GET() {
  // Get all blogs for dynamic sitemap
  const allBlogs = blogManager.getAllBlogs(1, 1000); // Get all blogs
  const allTags = blogManager.getAllTags();

  const staticPages = [
    { url: "/", priority: "1.0", changefreq: "weekly" },
    { url: "/about", priority: "0.8", changefreq: "monthly" },
    { url: "/admission", priority: "0.9", changefreq: "monthly" },
    { url: "/portfolio", priority: "0.7", changefreq: "weekly" },
    { url: "/blog", priority: "0.8", changefreq: "weekly" },
    { url: "/contact", priority: "0.6", changefreq: "monthly" },
  ];

  let sitemap = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">`;

  // Add static pages
  staticPages.forEach((page) => {
    sitemap += `
  <url>
    <loc>${SITE_URL}${page.url}</loc>
    <lastmod>${new Date().toISOString()}</lastmod>
    <changefreq>${page.changefreq}</changefreq>
    <priority>${page.priority}</priority>
  </url>`;
  });

  // Add blog posts (only if we have blogs)
  if (allBlogs.blogs.length > 0) {
    allBlogs.blogs.forEach((blog) => {
      sitemap += `
  <url>
    <loc>${SITE_URL}/blog/${blog.id}</loc>
    <lastmod>${new Date(blog.date).toISOString()}</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.7</priority>
  </url>`;
    });
  }

  // Add tag pages (only if we have tags)
  if (allTags.length > 0) {
    allTags.forEach((tag) => {
      sitemap += `
  <url>
    <loc>${SITE_URL}/blog?tag=${encodeURIComponent(tag)}</loc>
    <lastmod>${new Date().toISOString()}</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.5</priority>
  </url>`;
    });
  }

  sitemap += `
</urlset>`;

  return new Response(sitemap, {
    headers: {
      "Content-Type": "application/xml",
      "Cache-Control": "max-age=3600",
    },
  });
}
