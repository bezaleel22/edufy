<script>
// @ts-nocheck

  import { browser } from "$app/environment";
  import Slide from "$lib/components/Slide.svelte";
  import { waitVisible } from "$lib/utils";
  import { onMount } from "svelte";

  const slidesData = [
    {
      dataIndex: "rs-3051",
      title: "Lighthouse Leading Academy",
      description:
        "A uniquely designed educational facility exquisitely furnished and equipped to support.",
      src: "/images/slider/9.webp",
    },
    {
      dataIndex: "rs-3050",
      title: "Crèche Preparatory",
      description:
        " Early learning (Crèche Preparatory) which provides nurture and care to Infants, Toddlers and Pre-Schoolers.",
      src: "/images/slider/4.webp",
    },
    {
      dataIndex: "rs-3052",
      title: "Building the Next Generation.",
      description:
        "Nurturing and training the next generation by means of world class education.",
      src: "/images/slider/5.webp",
    },
    {
      dataIndex: "rs-3053",
      title: "Daycare, Crèche Preparatory",
      description:
        "Early learning (Daycare, Crèche Preparatory) which provides nurture and care to Infants, Toddlers and Pre-Schoolers.",
      src: "/images/slider/6.webp",
    },
  ];

  onMount(() => {
    let revapi = jQuery("#rev_slider_3")
      .show()
      .revolution({
        delay: 6000,
        responsiveLevels: [1200, 1140, 778, 480],
        gridwidth: [1220, 920, 700, 380],
        jsFileLocation: "js/",
        sliderLayout: "fullscreen",
        navigation: {
          keyboardNavigation: "off",
          keyboard_direction: "horizontal",
          mouseScrollNavigation: "off",
          onHoverStop: "off",
          bullets: {
            enable: true,
            style: "metis",
            hide_onmobile: true,
            hide_under: 700,
            h_align: "right",
            v_align: "bottom",
            h_offset: 180,
            hide_onleave: false,
            v_offset: 60,
            space: 15,
            tmp: '<span class="tp-bullet-img-wrap"><span class="tp-bullet-image"></span></span>',
          },
          arrows: { enable: false },
        },
      });
  });
</script>

<div class="rev_slider_wrapper">
  <div
    id="rev_slider_3"
    class="rev_slider fullwidthabanner"
    style="display:none;"
    data-version="5.4.1"
  >
    <ul class="w-100 h-100">
      {#each slidesData as slide}
        <Slide
          dataIndex={slide.dataIndex}
          title={slide.title}
          description={slide.description}
          src={slide.src}
        />
      {/each}
    </ul>
  </div>
</div>


