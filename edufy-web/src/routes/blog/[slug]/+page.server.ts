import type { PageServerLoad } from "./$types";
import { error } from "@sveltejs/kit";

interface BlogPost {
  id: string;
  title: string;
  slug: string;
  summary: string;
  body_html: string;
  author_id: string;
  tags: string[];
  date_published: string;
  visibility: string;
  cover_image?: string;
  attachments: string[];
  meta?: any;
}

export const load: PageServerLoad = async ({ params, platform, setHeaders, locals }) => {
  const { slug } = params;

  try {
    // Try to get data from KV if available (production)
    if (platform?.env?.BLOG_KV) {
      const blogPostData = await platform.env.BLOG_KV.get(`blog:post:${slug}`);
      
      if (blogPostData) {
        const blogPost: BlogPost = JSON.parse(blogPostData);
        
        // Check if post is private and user is not authenticated
        if (blogPost.visibility === 'private' && !locals.user) {
          throw error(404, 'Blog post not found');
        }

        // Set appropriate cache headers
        if (blogPost.visibility === 'private' || locals.user) {
          setHeaders({
            'Cache-Control': 'private, no-store'
          });
        } else {
          setHeaders({
            'Cache-Control': 'public, max-age=60, s-maxage=3600'
          });
        }

        return {
          post: blogPost
        };
      }
    }
    
    // Fallback: try to fetch from CMS API
    const cmsApiUrl = platform?.env?.CMS_API_URL || 'http://localhost:3001';
    try {
      // For private posts, we need to include authentication
      const headers: Record<string, string> = {
        'Accept': 'application/json',
      };

      // If user is authenticated, include session cookie for private post access
      if (locals.user) {
        // This would need the actual session token - for now use the admin endpoint
        const adminResponse = await fetch(`${cmsApiUrl}/api/admin/posts/${slug}`, {
          headers: {
            'Accept': 'application/json',
            // TODO: Include proper authentication headers
          },
        });
        
        if (adminResponse.ok) {
          const blogPost: BlogPost = await adminResponse.json();
          
          setHeaders({
            'Cache-Control': 'private, no-store'
          });
          
          return {
            post: blogPost
          };
        }
      } else {
        // For public posts only
        const response = await fetch(`${cmsApiUrl}/api/blog/post/${slug}`, {
          headers,
        });
        
        if (response.ok) {
          const blogPost: BlogPost = await response.json();
          
          setHeaders({
            'Cache-Control': 'public, max-age=60, s-maxage=3600'
          });
          
          return {
            post: blogPost
          };
        }
      }
    } catch (apiError) {
      console.error('Failed to fetch from CMS API:', apiError);
    }
    
    // Post not found
    throw error(404, 'Blog post not found');
    
  } catch (err) {
    console.error('Error loading blog post:', err);
    
    if (err && typeof err === 'object' && 'status' in err) {
      throw err; // Re-throw SvelteKit errors
    }
    
    throw error(500, 'Failed to load blog post');
  }
};
