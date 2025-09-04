<script lang="ts">
  import { page } from "$app/stores";
</script>

<svelte:head>
  <title>Page Not Found - Lighthouse Leading Academy</title>
  <meta
    name="description"
    content="The page you're looking for could not be found. Return to Lighthouse Leading Academy homepage."
  />
  <meta name="robots" content="noindex, nofollow" />
</svelte:head>

<section class="sidebarMenu">
  <div class="sidebarMenuOverlay"></div>
  <div class="SMArea">
    <div class="SMAHeader">
      <h3>
        <i class="twi-bars1"></i> Menu
      </h3>
      <a href={"#"} class="SMACloser"><i class="twi-times2"></i></a>
    </div>
    <div class="SMABody">
      <ul>
        <li class="menu-item-has-children current-menu-item">
          <a href={"#"}>Home</a>
          <ul class="sub-menu">
            <li><a href="/">Home</a></li>
            <li><a href="/">Home</a></li>
            <li><a href="/">Home</a></li>
          </ul>
        </li>
        <li class="menu-item-has-children">
          <a href={"#"}>About</a>
          <ul class="sub-menu">
            <li><a href="/about">About</a></li>
            <li><a href="/about">About</a></li>
          </ul>
        </li>
        <li class="menu-item-has-children">
          <a href={"#"}>blogs</a>
          <ul class="sub-menu">
            <li class="menu-item-has-children">
              <a href={"#"}>Blog List Views</a>
              <ul class="sub-menu">
                <li><a href="/about">About Us</a></li>
                <li><a href="/contact">Contact</a></li>
                <li><a href="/portfolio">Portfolio</a></li>
              </ul>
            </li>
            <li class="menu-item-has-children">
              <a href={"#"}>Blog Grid Views</a>
              <ul class="sub-menu">
                <li><a href="/admission">Admission</a></li>
                <li><a href="/portfolio">Portfolio</a></li>
                <li><a href="/contact">Contact</a></li>
              </ul>
            </li>
            <li><a href="/about">About Us</a></li>
          </ul>
        </li>
        <li class="menu-item-has-children">
          <a href={"#"}>pages</a>
          <ul class="sub-menu">
            <li class="menu-item-has-children">
              <a href={"#"}>Services</a>
              <ul class="sub-menu">
                <li><a href="/admission">Services</a></li>
                <li><a href="/admission">Services</a></li>
                <li><a href="/admission">Service Details</a></li>
              </ul>
            </li>
            <li class="menu-item-has-children">
              <a href={"#"}>Portfolios</a>
              <ul class="sub-menu">
                <li><a href="/portfolio">Portfolio</a></li>
                <li><a href="/portfolio">Portfolio</a></li>
                <li><a href="/portfolio">Portfolio Details</a></li>
              </ul>
            </li>
            <li><a href="/about">Team Page</a></li>
            <li><a href="/about">Company History</a></li>
            <li><a href="/about">404 Page</a></li>
          </ul>
        </li>
        <li><a href="/contact">Contact</a></li>
      </ul>
    </div>
  </div>
</section>

<section
  class="page_banner"
  style="background-image: url(/images/bg/banner.jpg);"
>
  <div class="container largeContainer">
    <div class="row">
      <div class="col-md-6">
        <h2 class="banner-title">Error Page</h2>
      </div>
      <div class="col-md-6 text-right">
        <p class="breadcrumbs">
          <a href="/" rel="v:url"><i class="twi-home-alt1"></i>Home</a><span
            >/</span
          >Error Page
        </p>
      </div>
    </div>
  </div>
</section>

<section class="section_404">
  <div class="container">
    <div class="row">
      <div class="col-lg-12">
        <div class="contetn_404 text-center">
          <img src="/images/404.png" alt={"404 Not Found"} />
          <h3>Oops! That page can't be found.</h3>
          <p>
            Unfortunately, something went wrong and this page does not exist.
            Try<br /> using the search or return to the previous page.
          </p>
          <a href="/" class="qu_btn">Go Back To Home</a>
        </div>
      </div>
    </div>
  </div>
</section>
