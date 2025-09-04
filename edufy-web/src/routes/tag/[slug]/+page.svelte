<script lang="ts">
  import type { Blog } from '$lib/data/blog';
  
  export let data: {
    tag: string;
    posts: Blog[];
    allTags: string[];
    recentPosts: Blog[];
  };
</script>

<svelte:head>
  <title>Tag: {data.tag} - LL Academy</title>
  <meta name="description" content="Posts tagged with {data.tag}" />
</svelte:head>

<section class="page_banner blog_page_banner">
  <div class="boverlay"></div>
  <div class="container largeContainer">
    <div class="row">
      <div class="col-md-7">
        <h2 class="banner-title">Tag: {data.tag}</h2>
      </div>
      <div class="col-md-5 text-right">
        <p class="breadcrumbs">
          <a href="/" rel="v:url">
            <i class="twi-home-alt1"></i>Home
          </a>
          <span>/</span>
          <a href="/blog">Blog</a>
          <span>/</span>
          Tag: {data.tag}
        </p>
      </div>
    </div>
  </div>
</section>

<section class="blogPage">
  <div class="container largeContainer">
    <div class="row">
      <div class="col-lg-8">
        {#if data.posts.length > 0}
          {#each data.posts as post}
            <div class="bloglistItem">
              <div class="blThumb">
                <img src={post.image} alt={post.title.substring(0, 10)} />
              </div>
              <div class="blogContent02">
                <div class="bmeta">
                  <a href="/blog/{post.id}"><i class="twi-eye2"></i>{post.views}</a>
                  <a href="/blog/{post.id}"><i class="twi-comments2"></i>{post.comments}</a>
                  <a href="/blog/{post.id}"><i class="twi-calendar-alt2"></i>{post.date}</a>
                </div>
                <h3>
                  <a href="/blog/{post.id}">{post.title}</a>
                </h3>
                <p>
                  {post.description}
                </p>
                <a href="/blog/{post.id}" class="rm_more">
                  Read More
                  <i class="twi-long-arrow-right1"></i>
                </a>
              </div>
            </div>
          {/each}
        {:else}
          <div class="text-center">
            <h3>No posts found with tag "{data.tag}"</h3>
            <p>Try browsing our <a href="/blog">latest blog posts</a> instead.</p>
          </div>
        {/if}
      </div>
      
      <div class="col-lg-4">
        <div class="sidebar">
          <aside class="widget custome_categories">
            <h3 class="widget_title">Popular Tags</h3>
            <div class="tag-cloud">
              {#each data.allTags as tag}
                <a href="/tag/{tag}" class="tag-link">{tag}</a>
              {/each}
            </div>
          </aside>
          
          <aside class="widget qeura_rp_widget">
            <h3 class="widget_title">Recent Posts</h3>
            {#each data.recentPosts as post}
              <div class="pp_post_item clearfix">
                <a href="/blog/{post.id}">
                  <img src={post.image} alt={post.title} />
                </a>
                <a class="pptitle" href="/blog/{post.id}">
                  {post.title.substring(0, 40)}...
                </a>
                <span><i class="twi-calendar-alt2"></i>{post.date}</span>
              </div>
            {/each}
          </aside>
        </div>
      </div>
    </div>
  </div>
</section>

<style>
  .tag-cloud {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }
  
  .tag-link {
    background: #f8f9fa;
    padding: 4px 12px;
    border-radius: 4px;
    text-decoration: none;
    color: #333;
    font-size: 14px;
    transition: all 0.3s ease;
  }
  
  .tag-link:hover {
    background: #007bff;
    color: white;
    text-decoration: none;
  }
</style>