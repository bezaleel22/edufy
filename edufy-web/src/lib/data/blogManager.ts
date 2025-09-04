import type { Blog } from './blog';
import { blogs } from './blog';

/**
 * Blog Manager for efficient data handling and caching
 */
export class BlogManager {
  private static instance: BlogManager;
  private blogCache = new Map<number, Blog>();
  private tagCache = new Map<string, Blog[]>();
  private allTags: string[] = [];
  private initialized = false;

  private constructor() {}

  static getInstance(): BlogManager {
    if (!BlogManager.instance) {
      BlogManager.instance = new BlogManager();
    }
    return BlogManager.instance;
  }

  private initialize() {
    if (this.initialized) return;

    // Cache all blogs
    blogs.forEach(blog => {
      this.blogCache.set(blog.id, blog);
    });

    // Build tag cache
    const tagSet = new Set<string>();
    blogs.forEach(blog => {
      blog.tags.forEach(tag => {
        tagSet.add(tag);
        
        if (!this.tagCache.has(tag)) {
          this.tagCache.set(tag, []);
        }
        this.tagCache.get(tag)!.push(blog);
      });
    });

    this.allTags = Array.from(tagSet).sort();
    this.initialized = true;
  }

  /**
   * Get all blogs with pagination
   */
  getAllBlogs(page = 1, limit = 10): { blogs: Blog[], total: number, hasMore: boolean } {
    this.initialize();
    
    const allBlogs = Array.from(this.blogCache.values())
      .sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());
    
    const start = (page - 1) * limit;
    const end = start + limit;
    const paginatedBlogs = allBlogs.slice(start, end);
    
    return {
      blogs: paginatedBlogs,
      total: allBlogs.length,
      hasMore: end < allBlogs.length
    };
  }

  /**
   * Get blog by ID
   */
  getBlogById(id: number): Blog | null {
    this.initialize();
    return this.blogCache.get(id) || null;
  }

  /**
   * Get blogs by tag
   */
  getBlogsByTag(tag: string, page = 1, limit = 10): { blogs: Blog[], total: number, hasMore: boolean } {
    this.initialize();
    
    const tagBlogs = this.tagCache.get(tag) || [];
    const sortedBlogs = tagBlogs.sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());
    
    const start = (page - 1) * limit;
    const end = start + limit;
    const paginatedBlogs = sortedBlogs.slice(start, end);
    
    return {
      blogs: paginatedBlogs,
      total: sortedBlogs.length,
      hasMore: end < sortedBlogs.length
    };
  }

  /**
   * Get all tags
   */
  getAllTags(): string[] {
    this.initialize();
    return [...this.allTags];
  }

  /**
   * Get recent blogs
   */
  getRecentBlogs(limit = 5): Blog[] {
    this.initialize();
    
    return Array.from(this.blogCache.values())
      .sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime())
      .slice(0, limit);
  }

  /**
   * Search blogs
   */
  searchBlogs(query: string, page = 1, limit = 10): { blogs: Blog[], total: number, hasMore: boolean } {
    this.initialize();
    
    const searchTerm = query.toLowerCase();
    const matchingBlogs = Array.from(this.blogCache.values())
      .filter(blog => 
        blog.title.toLowerCase().includes(searchTerm) ||
        blog.description.toLowerCase().includes(searchTerm) ||
        blog.tags.some(tag => tag.toLowerCase().includes(searchTerm))
      )
      .sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());
    
    const start = (page - 1) * limit;
    const end = start + limit;
    const paginatedBlogs = matchingBlogs.slice(start, end);
    
    return {
      blogs: paginatedBlogs,
      total: matchingBlogs.length,
      hasMore: end < matchingBlogs.length
    };
  }

  /**
   * Get blog statistics
   */
  getStats(): { totalBlogs: number, totalTags: number, totalViews: number } {
    this.initialize();
    
    const totalBlogs = this.blogCache.size;
    const totalTags = this.allTags.length;
    const totalViews = Array.from(this.blogCache.values())
      .reduce((sum, blog) => sum + blog.views, 0);
    
    return { totalBlogs, totalTags, totalViews };
  }
}

// Export singleton instance
export const blogManager = BlogManager.getInstance();
