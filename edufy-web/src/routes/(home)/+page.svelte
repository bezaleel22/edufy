<script lang="ts">
  import About from "./about.svelte";
  import Features from "./blogs.svelte";
  import Call2action from "./call2action.svelte";
  import Partnership from "./partnership.svelte";
  import Potfolio from "./potfolio.svelte";
  import Services from "./services.svelte";
  import Slider from "./slider.svelte";
  import Sponsorship from "./sponsorship.svelte";
  import Whyus from "./whyus.svelte";
</script>

<section class="slider_03">
  <Slider />
</section>

<section class="aboutSection03">
  <About />
</section>

<section class="serviceSection03">
  <Services />
</section>

<section id="sponsor" class="chooseSection chooseSection02">
  <Sponsorship />
</section>

<section class="appoinmentSection03">
  <Partnership />
</section>

<section class="teamSection02">
  <div class="container largeContainer">
    <div class="row">
      <div class="col-lg-12 text-center">
        <div class="subTitle">
          <span class="bleft"></span>
          Meet Our Exceptional Instructors
          <span class="bright"></span>
        </div>
        <h2 class="secTitle">Expert<span>Team</span></h2>
      </div>
    </div>
  </div>
</section>

<section class="ctaSection">
  <Call2action />
</section>

<section class="chooseSection03">
  <Whyus />
</section>

<section class="portfolioSection03">
  <Potfolio />
</section>

<section class="blogSectiont03">
  <Features />
</section>
