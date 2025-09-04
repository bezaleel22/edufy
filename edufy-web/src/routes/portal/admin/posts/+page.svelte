<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  
  export let data: any;
  
  const { posts, user } = data;
  
  async function deletePost(slug: string) {
    if (!confirm('Are you sure you want to delete this post?')) {
      return;
    }
    
    try {
      const response = await fetch(`/api/admin/posts/${slug}`, {
        method: 'DELETE',
        credentials: 'include'
      });
      
      if (response.ok) {
        // Refresh the page to update the list
        window.location.reload();
      } else {
        alert('Failed to delete post');
      }
    } catch (error) {
      console.error('Delete error:', error);
      alert('Failed to delete post');
    }
  }
  
  function formatDate(dateString: string) {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric'
    });
  }
</script>

<svelte:head>
  <title>Manage Posts - LLA Admin</title>
</svelte:head>

<div class="admin-posts">
  <div class="admin-header">
    <h1>Manage Blog Posts</h1>
    <a href="/portal/admin/posts/new" class="btn btn-primary">Create New Post</a>
  </div>
  
  {#if posts && posts.length > 0}
    <div class="posts-table">
      <table>
        <thead>
          <tr>
            <th>Title</th>
            <th>Slug</th>
            <th>Visibility</th>
            <th>Published</th>
            <th>Tags</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each posts as post}
            <tr>
              <td>
                <div class="post-title">
                  {post.title}
                  {#if post.summary}
                    <div class="post-summary">{post.summary}</div>
                  {/if}
                </div>
              </td>
              <td>
                <code class="slug">{post.slug}</code>
              </td>
              <td>
                <span class="visibility-badge" class:public={post.visibility === 'public'} class:private={post.visibility === 'private'}>
                  {post.visibility}
                </span>
              </td>
              <td>{formatDate(post.date_published)}</td>
              <td>
                <div class="tags">
                  {#each post.tags || [] as tag}
                    <span class="tag">{tag}</span>
                  {/each}
                </div>
              </td>
              <td>
                <div class="actions">
                  <a href="/blog/{post.slug}" target="_blank" class="btn btn-small btn-outline">View</a>
                  <a href="/portal/admin/posts/{post.slug}/edit" class="btn btn-small btn-secondary">Edit</a>
                  <button on:click={() => deletePost(post.slug)} class="btn btn-small btn-danger">Delete</button>
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {:else}
    <div class="empty-state">
      <h2>No posts yet</h2>
      <p>Create your first blog post to get started.</p>
      <a href="/portal/admin/posts/new" class="btn btn-primary">Create New Post</a>
    </div>
  {/if}
</div>

<style>
  .admin-posts {
    padding: 0;
  }
  
  .admin-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid #e1e8ed;
  }
  
  .admin-header h1 {
    margin: 0;
    color: #2c3e50;
  }
  
  .btn {
    display: inline-block;
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 4px;
    text-decoration: none;
    font-size: 0.9rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    text-align: center;
  }
  
  .btn-primary {
    background-color: #3498db;
    color: white;
  }
  
  .btn-primary:hover {
    background-color: #2980b9;
  }
  
  .btn-secondary {
    background-color: #95a5a6;
    color: white;
  }
  
  .btn-secondary:hover {
    background-color: #7f8c8d;
  }
  
  .btn-danger {
    background-color: #e74c3c;
    color: white;
  }
  
  .btn-danger:hover {
    background-color: #c0392b;
  }
  
  .btn-outline {
    background-color: transparent;
    color: #3498db;
    border: 1px solid #3498db;
  }
  
  .btn-outline:hover {
    background-color: #3498db;
    color: white;
  }
  
  .btn-small {
    padding: 0.25rem 0.5rem;
    font-size: 0.8rem;
  }
  
  .posts-table {
    background: white;
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }
  
  table {
    width: 100%;
    border-collapse: collapse;
  }
  
  th, td {
    padding: 1rem;
    text-align: left;
    border-bottom: 1px solid #e1e8ed;
  }
  
  th {
    background-color: #f8f9fa;
    font-weight: 600;
    color: #2c3e50;
  }
  
  tbody tr:hover {
    background-color: #f8f9fa;
  }
  
  .post-title {
    font-weight: 600;
    color: #2c3e50;
  }
  
  .post-summary {
    font-size: 0.85rem;
    color: #7f8c8d;
    margin-top: 0.25rem;
  }
  
  .slug {
    background-color: #f1f2f6;
    padding: 0.25rem 0.5rem;
    border-radius: 3px;
    font-family: 'Courier New', monospace;
    font-size: 0.8rem;
  }
  
  .visibility-badge {
    padding: 0.25rem 0.5rem;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
  }
  
  .visibility-badge.public {
    background-color: #d4edda;
    color: #155724;
  }
  
  .visibility-badge.private {
    background-color: #f8d7da;
    color: #721c24;
  }
  
  .tags {
    display: flex;
    flex-wrap: wrap;
    gap: 0.25rem;
  }
  
  .tag {
    background-color: #e9ecef;
    color: #495057;
    padding: 0.125rem 0.375rem;
    border-radius: 3px;
    font-size: 0.75rem;
  }
  
  .actions {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }
  
  .empty-state {
    text-align: center;
    padding: 4rem 2rem;
    color: #7f8c8d;
  }
  
  .empty-state h2 {
    margin-bottom: 1rem;
    color: #2c3e50;
  }
  
  .empty-state p {
    margin-bottom: 2rem;
  }
  
  @media (max-width: 768px) {
    .admin-header {
      flex-direction: column;
      align-items: stretch;
      gap: 1rem;
    }
    
    .posts-table {
      overflow-x: auto;
    }
    
    table {
      min-width: 800px;
    }
    
    .actions {
      flex-direction: column;
    }
  }
</style>
