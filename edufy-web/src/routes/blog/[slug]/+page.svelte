<script lang="ts">
  import type { PageData } from './$types';
  import SEO from '$lib/components/SEO.svelte';

  export let data: PageData;
  $: post = data.post;
</script>

<SEO 
  title={post.title} 
  description={post.summary} 
  image={post.cover_image}
  type="article"
/>

<section class="blog-single-content section-padding">
  <div class="container">
    <div class="row">
      <div class="col-lg-8">
        <article class="blog-post">
          {#if post.cover_image}
            <div class="post-thumbnail">
              <img src={post.cover_image} alt={post.title} class="img-fluid" />
            </div>
          {/if}

          <div class="post-meta">
            <span class="post-date">
              {new Date(post.date_published).toLocaleDateString('en-US', {
                year: 'numeric',
                month: 'long',
                day: 'numeric'
              })}
            </span>
            {#if post.tags && post.tags.length > 0}
              <div class="post-tags">
                {#each post.tags as tag}
                  <a href="/tag/{tag}" class="tag-link">#{tag}</a>
                {/each}
              </div>
            {/if}
          </div>

          <div class="post-content">
            {@html post.body_html}
          </div>

          {#if post.attachments && post.attachments.length > 0}
            <div class="post-attachments">
              <h4>Attachments</h4>
              <ul>
                {#each post.attachments as attachment}
                  <li>
                    <a href={attachment} target="_blank" rel="noopener noreferrer">
                      Download Attachment
                    </a>
                  </li>
                {/each}
              </ul>
            </div>
          {/if}
        </article>
      </div>

      <div class="col-lg-4">
        <aside class="blog-sidebar">
          <div class="widget">
            <h4>Recent Posts</h4>
            <!-- TODO: Add recent posts widget -->
          </div>

          <div class="widget">
            <h4>Categories</h4>
            <!-- TODO: Add categories widget -->
          </div>
        </aside>
      </div>
    </div>
  </div>
</section>

<style>
  .blog-single-content {
    padding: 80px 0;
  }

  .blog-post {
    background: #fff;
    padding: 40px;
    border-radius: 8px;
    box-shadow: 0 2px 10px rgba(0,0,0,0.1);
    margin-bottom: 40px;
  }

  .post-thumbnail {
    margin-bottom: 30px;
  }

  .post-thumbnail img {
    width: 100%;
    height: auto;
    border-radius: 8px;
  }

  .post-meta {
    margin-bottom: 30px;
    padding-bottom: 20px;
    border-bottom: 1px solid #eee;
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
  }

  .post-date {
    color: #666;
    font-size: 14px;
  }

  .post-tags {
    display: flex;
    gap: 10px;
    flex-wrap: wrap;
  }

  .tag-link {
    background: #f0f0f0;
    padding: 4px 8px;
    border-radius: 4px;
    text-decoration: none;
    color: #666;
    font-size: 12px;
    transition: background-color 0.3s;
  }

  .tag-link:hover {
    background: #e0e0e0;
    color: #333;
  }

  .post-content {
    line-height: 1.8;
    font-size: 16px;
  }

  .post-content :global(h1),
  .post-content :global(h2),
  .post-content :global(h3),
  .post-content :global(h4),
  .post-content :global(h5),
  .post-content :global(h6) {
    margin-top: 30px;
    margin-bottom: 15px;
  }

  .post-content :global(p) {
    margin-bottom: 20px;
  }

  .post-content :global(img) {
    max-width: 100%;
    height: auto;
    border-radius: 4px;
    margin: 20px 0;
  }

  .post-attachments {
    margin-top: 40px;
    padding-top: 30px;
    border-top: 1px solid #eee;
  }

  .post-attachments h4 {
    margin-bottom: 15px;
  }

  .post-attachments ul {
    list-style: none;
    padding: 0;
  }

  .post-attachments li {
    margin-bottom: 10px;
  }

  .post-attachments a {
    color: #007bff;
    text-decoration: none;
    padding: 8px 16px;
    background: #f8f9fa;
    border-radius: 4px;
    display: inline-block;
    transition: background-color 0.3s;
  }

  .post-attachments a:hover {
    background: #e9ecef;
  }

  .blog-sidebar {
    padding-left: 30px;
  }

  .widget {
    background: #fff;
    padding: 30px;
    border-radius: 8px;
    box-shadow: 0 2px 10px rgba(0,0,0,0.1);
    margin-bottom: 30px;
  }

  .widget h4 {
    margin-bottom: 20px;
    padding-bottom: 15px;
    border-bottom: 2px solid #007bff;
  }

  @media (max-width: 991px) {
    .blog-sidebar {
      padding-left: 0;
      margin-top: 40px;
    }

    .post-meta {
      flex-direction: column;
      align-items: flex-start;
      gap: 10px;
    }

    .blog-post {
      padding: 20px;
    }
  }
</style>
