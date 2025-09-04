import type { PageServerLoad } from "./$types";

interface BlogIndexEntry {
  slug: string;
  title: string;
  summary: string;
  cover_image?: string;
  date_published: string;
  tags: string[];
  visibility: string;
}

export const load: PageServerLoad = async ({ platform, url, setHeaders }) => {
  try {
    // Set cache headers for public blog index
    setHeaders({
      'Cache-Control': 'public, max-age=60, s-maxage=3600'
    });

    // Try to get data from KV if available (production)
    if (platform?.env?.BLOG_KV) {
      const blogIndexData = await platform.env.BLOG_KV.get('blog:index');
      if (blogIndexData) {
        const blogIndex: BlogIndexEntry[] = JSON.parse(blogIndexData);
        
        // Filter only public posts
        const publicBlogs = blogIndex.filter(blog => blog.visibility === 'public');
        
        // Get all unique tags
        const allTags = [...new Set(publicBlogs.flatMap(blog => blog.tags))];
        
        return {
          blogs: publicBlogs,
          pagination: {
            currentPage: 1,
            totalBlogs: publicBlogs.length,
            hasMore: false,
            limit: 50,
          },
          allTags,
          currentSearch: "",
          currentTag: "",
        };
      }
    }
    
    // Fallback: try to fetch from CMS API
    const cmsApiUrl = platform?.env?.CMS_API_URL || 'http://localhost:3001';
    try {
      const response = await fetch(`${cmsApiUrl}/api/blog/index`, {
        headers: {
          'Accept': 'application/json',
        },
      });
      
      if (response.ok) {
        const blogIndex: BlogIndexEntry[] = await response.json();
        const allTags = [...new Set(blogIndex.flatMap(blog => blog.tags))];
        
        return {
          blogs: blogIndex,
          pagination: {
            currentPage: 1,
            totalBlogs: blogIndex.length,
            hasMore: false,
            limit: 50,
          },
          allTags,
          currentSearch: "",
          currentTag: "",
        };
      }
    } catch (apiError) {
      console.error('Failed to fetch from CMS API:', apiError);
    }
    
    // Final fallback: return empty data
    return {
      blogs: [],
      pagination: {
        currentPage: 1,
        totalBlogs: 0,
        hasMore: false,
        limit: 10,
      },
      allTags: [],
      currentSearch: "",
      currentTag: "",
    };
    
  } catch (error) {
    console.error('Error loading blog data:', error);
    
    // Return empty data on error
    return {
      blogs: [],
      pagination: {
        currentPage: 1,
        totalBlogs: 0,
        hasMore: false,
        limit: 10,
      },
      allTags: [],
      currentSearch: "",
      currentTag: "",
    };
  }
};
