import { error } from '@sveltejs/kit';
import type { ServerLoad } from '@sveltejs/kit';

export const load: ServerLoad = async ({ locals, platform, fetch }) => {
  // Check if user is admin (already verified in hooks)
  if (!locals.user || locals.user.role !== 'admin') {
    throw error(403, 'Access denied');
  }

  try {
    // Fetch all posts (including private) from CMS API
    const cmsApiUrl = platform?.env?.CMS_API_URL || 'http://localhost:8080';
    
    const response = await fetch(`${cmsApiUrl}/api/admin/posts`, {
      headers: {
        'Cookie': `session=${locals.token || ''}`,
        'Accept': 'application/json',
      },
    });

    if (response.ok) {
      const posts = await response.json();
      return {
        posts,
        user: locals.user
      };
    } else {
      console.error('Failed to fetch posts:', response.status);
      return {
        posts: [],
        user: locals.user
      };
    }
  } catch (err) {
    console.error('Error fetching posts:', err);
    return {
      posts: [],
      user: locals.user
    };
  }
};
