<script>
  import Largecontainer from "./largecontainer.svelte";
  import Portfoliopage from "./portfoliopage.svelte";
  import Smarea from "./smarea.svelte";
</script>

<section class="sidebarMenu">
  <div class="sidebarMenuOverlay"></div>
  <Smarea />
</section>

<section
  class="page_banner"
  style="background-image: url(/images/bg/banner.jpg);"
>
  <Largecontainer />
</section>

<section class="portfolioPage">
  <Portfoliopage />
</section>
