<script lang="ts">
  import type { PageData } from "./$types";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";

  export let data: PageData;

  $: blogs = data.blogs || [];
  $: pagination = data.pagination;
  $: allTags = data.allTags || [];
  $: currentSearch = data.currentSearch || "";
  $: currentTag = data.currentTag || "";

  $: console.log({ blogs });

  let searchQuery = currentSearch;

  function handleSearch() {
    const params = new URLSearchParams();
    if (searchQuery.trim()) {
      params.set("search", searchQuery.trim());
    }
    if (currentTag) {
      params.set("tag", currentTag);
    }
    params.set("page", "1");

    goto(`/blog?${params.toString()}`);
  }

  function handleTagFilter(tag: string) {
    const params = new URLSearchParams();
    if (tag !== currentTag) {
      params.set("tag", tag);
    }
    if (searchQuery.trim()) {
      params.set("search", searchQuery.trim());
    }
    params.set("page", "1");

    goto(`/blog?${params.toString()}`);
  }

  function handlePageChange(newPage: number) {
    const params = new URLSearchParams($page.url.searchParams);
    params.set("page", newPage.toString());

    goto(`/blog?${params.toString()}`);
  }

  function clearFilters() {
    searchQuery = "";
    goto("/blog");
  }

  function formatDate(dateString: string): string {
    try {
      return new Date(dateString).toLocaleDateString("en-US", {
        year: "numeric",
        month: "short",
        day: "numeric",
      });
    } catch {
      return dateString;
    }
  }
</script>

<section class="page_banner blog_page_banner" style="">
  <div class="boverlay" style=""></div>
  <div class="container largeContainer">
    <div class="row">
      <div class="col-md-7">
        <h2 class="banner-title">Blog &amp; News</h2>
      </div>
      <div class="col-md-5 text-right">
        <p class="breadcrumbs">
          <a href="/" rel="v:url"><i class="twi-home-alt1"></i>Home</a><span
            >/</span
          >Blog & News
        </p>
      </div>
    </div>
  </div>
</section>

<section class="blogPage">
  <div class="container largeContainer">
    <div class="row">
      <div class="col-lg-8">
        {#each blogs as post}
          <div class="bloglistItem">
            <div class="blThumb">
              <img
                src={post.cover_image || "/images/blog/default.jpg"}
                alt={post.title.substring(0, 10)}
              />
            </div>
            <div class="blogContent02">
              <div class="bmeta">
                <a href="/blog/{post.slug}"><i class="twi-eye2"></i>0</a>
                <a href="/blog/{post.slug}"><i class="twi-comments2"></i>0</a>
                <a href="/blog/{post.slug}"
                  ><i class="twi-calendar-alt2"></i>{formatDate(
                    post.date_published
                  )}</a
                >
              </div>
              <h3>
                <a href="/blog/{post.slug}">{post.title.substring(0, 70)}</a>
              </h3>
              <p>
                {post.summary}
              </p>
              <a href="/blog/{post.slug}" class="rm_more">
                Read More
                <i class="twi-long-arrow-right1"> </i>
              </a>
            </div>
          </div>
        {/each}
        <div class="row">
          <div class="col-lg-12">
            <div class="que_pagination text-center">
              <nav class="navigation pagination" aria-label="Posts">
                <h2 class="screen-reader-text">Posts navigation</h2>
                <div class="nav-links">
                  <span aria-current="page" class="page-numbers current">1</span
                  >
                  <a class="page-numbers" href="/blog">2</a>
                  <a class="next page-numbers" href="/blog"
                    ><i class="twi-angle-double-right1"></i></a
                  >
                </div>
              </nav>
            </div>
          </div>
        </div>
      </div>
      <div class="col-lg-4">
        <div class="sidebar">
          <aside id="block-7" class="widget widget_block widget_search">
            <form
              role="search"
              method="get"
              action="/blog"
              class="wp-block-search__button-outside wp-block-search__icon-button wp-block-search"
            >
              <label
                class="wp-block-search__label"
                for="wp-block-search__input-1">Search</label
              >
              <div class="wp-block-search__inside-wrapper" style="width: 666px">
                <input
                  class="wp-block-search__input"
                  id="wp-block-search__input-1"
                  placeholder=""
                  value=""
                  type="search"
                  name="s"
                /><button
                  aria-label="Search"
                  class="wp-block-search__button has-icon wp-element-button"
                  type="submit"
                  ><svg
                    class="search-icon"
                    viewBox="0 0 24 24"
                    width="24"
                    height="24"
                  >
                    <path
                      d="M13 5c-3.3 0-6 2.7-6 6 0 1.4.5 2.7 1.3 3.7l-3.8 3.8 1.1 1.1 3.8-3.8c1 .8 2.3 1.3 3.7 1.3 3.3 0 6-2.7 6-6S16.3 5 13 5zm0 10.5c-2.5 0-4.5-2-4.5-4.5s2-4.5 4.5-4.5 4.5 2 4.5 4.5-2 4.5-4.5 4.5z"
                    ></path>
                  </svg></button
                >
              </div>
            </form>
          </aside>
          <aside id="quera-categories-2" class="widget custome_categories">
            <h3 class="widget_title">Categories</h3>
            <ul class="clearfix">
              <li class="clearfix">
                <a href="/blog">Business Growth</a>
                <span>2</span>
              </li>
              <li class="clearfix">
                <a href="/blog">Financial Work</a>
                <span>2</span>
              </li>
              <li class="clearfix">
                <a href="/blog">Market Strategy</a>
                <span>2</span>
              </li>
              <li class="clearfix">
                <a href="/blog">SEO Marketing</a>
                <span>2</span>
              </li>
            </ul>
          </aside>
          <aside id="mak-rppwt-2" class="widget qeura_rp_widget">
            <h3 class="widget_title">Popular Posts</h3>
            <div class="pp_post_item clearfix">
              <a href="/blog/1"
                ><img
                  src="https://themewar.com/wp/quera/wp-content/uploads/2022/01/b1-90x80.jpg"
                  alt="Efficiently monetize models transparent sources redefine distributed"
                /></a
              >
              <a class="pptitle" href="/blog/1"
                >Efficiently monetize models transparent
              </a>
              <span><i class="twi-calendar-alt2"></i>Jan 25, 2022</span>
            </div>
            <div class="pp_post_item clearfix">
              <a href="/blog/2"
                ><img
                  src="https://themewar.com/wp/quera/wp-content/uploads/2022/01/b4-90x80.jpg"
                  alt="How to Optimize Your SEO Marketing Strategy Search Engine Optimization"
                /></a
              >
              <a class="pptitle" href="/blog/2"
                >How to Optimize Your SEO Marketing Strat</a
              >
              <span><i class="twi-calendar-alt2"></i>Jan 25, 2022</span>
            </div>
            <div class="pp_post_item clearfix">
              <a href="/blog/3"
                ><img
                  src="https://themewar.com/wp/quera/wp-content/uploads/2022/01/b3-90x80.jpg"
                  alt="Are Pinterest Stories Right for Your Marketing Strategy?"
                /></a
              >
              <a class="pptitle" href="/blog/3"
                >Are Pinterest Stories Right for Your Mar</a
              >
              <span><i class="twi-calendar-alt2"></i>Jan 25, 2022</span>
            </div>
          </aside>
          <aside id="block-8" class="widget widget_block widget_tag_cloud">
            <p class="wp-block-tag-cloud">
              <a
                href="/blog"
                class="tag-cloud-link tag-link-6 tag-link-position-1"
                style="font-size: 8pt;"
                aria-label="Business (4 items)">Business</a
              >
              <a
                href="/blog"
                class="tag-cloud-link tag-link-8 tag-link-position-2"
                style="font-size: 8pt;"
                aria-label="Focus (4 items)">Focus</a
              >
              <a
                href="/blog"
                class="tag-cloud-link tag-link-7 tag-link-position-3"
                style="font-size: 8pt;"
                aria-label="Growth (4 items)">Growth</a
              >
              <a
                href="/blog"
                class="tag-cloud-link tag-link-9 tag-link-position-4"
                style="font-size: 8pt;"
                aria-label="Service (4 items)">Service</a
              >
              <a
                href="/blog"
                class="tag-cloud-link tag-link-10 tag-link-position-5"
                style="font-size: 8pt;"
                aria-label="Tools (4 items)">Tools</a
              >
              <a
                href="/blog"
                class="tag-cloud-link tag-link-11 tag-link-position-6"
                style="font-size: 8pt;"
                aria-label="UI/UX (4 items)">UI/UX</a
              >
            </p>
          </aside>
        </div>
      </div>
    </div>
  </div>
</section>
