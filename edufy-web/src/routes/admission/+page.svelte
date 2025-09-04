<script>
  import Container from "./container.svelte";
  import Pagebanner from "./pagebanner.svelte";
  import Process from "./process.svelte";
  import Testimonial from "./testimonial.svelte";
  import Portfolio from "./portfolio.svelte";
  import Largecontainer from "./largecontainer.svelte";
  import Largecontainer2 from "./largecontainer2.svelte";
</script>

<section
  class="page_banner"
  style="background-image: url(/images/bg/banner.jpg);"
>
  <Pagebanner />
</section>

<section class="processSection01">
  <Process />
</section>

<section class="chooseSection">
  <Container />
</section>

<section class="testimonialSection01">
  <Testimonial />
</section>

<section class="portfolioSection01">
  <Portfolio />
</section>

<section class="serviceSection01">
  <Largecontainer />
</section>

<section class="appoinmentSection01">
  <Largecontainer2 />
</section>
