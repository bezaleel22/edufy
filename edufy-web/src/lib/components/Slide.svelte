<script lang="ts">
  import { frames } from "$lib/data/frames";

  export let dataIndex;
  export let src;
  export let title;
  export let description;
  export let buttonText = "Enroll Now";
  export let buttonLink = "/admission";
</script>

<li
  data-index={dataIndex}
  data-transition="random-premium"
  data-slotamount="default"
  data-hideafterloop="0"
  data-hideslideonmobile="off"
  data-easein="Power3.easeInOut"
  data-easeout="Power3.easeInOut"
  data-masterspeed="1000"
  data-thumb=""
  data-rotate="0"
  data-saveperformance="off"
  data-title=""
  data-param1=""
  data-param2=""
  data-param3=""
  data-param4=""
  data-param5=""
  data-param6=""
  data-param7=""
  data-param8=""
  data-param9=""
  data-param10=""
  data-description=""
>
  <img
    {src}
    alt="Quera"
    data-bgposition="center center"
    data-bgfit="cover"
    data-bgrepeat="no-repeat"
    data-bgparallax="0"
    class="rev-slidebg"
    data-no-retina
  />
  <div
    class="tp-caption headFont ws_nowrap"
    data-x="['left']"
    data-hoffset="['0']"
    data-y="['middle']"
    data-voffset="['8','8','-10','-20']"
    data-fontsize="['90','80','60','36']"
    data-fontweight="['700']"
    data-lineheight="['100','90','74','46']"
    data-letterspacing="0"
    data-width="['100%']"
    data-height="['auto']"
    data-whitesapce="['normal']"
    data-color="['#fff']"
    data-type="text"
    data-responsive_offset="off"
    data-frames={JSON.stringify(frames.title)}
    data-textAlign="['left']"
    data-paddingtop="[0,0,0,0]"
    data-paddingright="[0,0,0,20]"
    data-paddingbottom="[0,0,0,0]"
    data-paddingleft="[0,0,0,20]"
  >
    <div class="w-50">
      {title}
    </div>
  </div>
  <div
    class="tp-caption ws_nowrap"
    data-x="['left']"
    data-hoffset="['0']"
    data-y="['middle']"
    data-voffset="['158','158','120','100']"
    data-fontsize="['18','18','18','18']"
    data-fontweight="['400']"
    data-lineheight="['30','30','30','30']"
    data-letterspacing="0"
    data-width="['750','750','600','100%']"
    data-height="['auto']"
    data-whitesapce="['normal']"
    data-color="['#d7d7d7']"
    data-type="text"
    data-responsive_offset="off"
    data-frames={JSON.stringify(frames.description)}
    data-textAlign="['left']"
    data-paddingtop="[0,0,0,0]"
    data-paddingright="[0,0,0,20]"
    data-paddingbottom="[0,0,0,0]"
    data-paddingleft="[0,0,0,20]"
  >
    <span class="font-weight-bold">{description}</span>
  </div>

  <div
    class="tp-caption tp-resizeme"
    data-x="['left']"
    data-hoffset="['0']"
    data-y="['middle']"
    data-voffset="['266','266','230','230']"
    data-fontsize="['16']"
    data-fontweight="700"
    data-lineheight="60"
    data-width="['auto']"
    data-height="['auto']"
    data-whitesapce="['normal']"
    data-color="['#fff']"
    data-type="text"
    data-responsive_offset="off"
    data-frames={JSON.stringify(frames.button)}
    data-textAlign="['center']"
    data-paddingtop="[0,0,0,0]"
    data-paddingright="[0,0,0,0]"
    data-paddingbottom="[0,0,0,0]"
    data-paddingleft="[0,0,0,20]"
  >
    <a href={buttonLink} class="qu_btn">{buttonText}</a>
  </div>

  <div
    class="tp-caption tp-resizeme anLayer"
    data-frames={JSON.stringify(frames.image1)}
    data-type="image"
    data-x="right"
    data-y="bottom"
    data-hoffset="['-250','0','0','0']"
    data-voffset="['-300','0','0','0']"
    data-width="['auto']"
    data-height="['auto']"
  >
    <img src="/images/slider/s7.png" alt="layer" width="844" height="731" />
  </div>

  <div
    class="tp-caption tp-resizeme anLayer"
    data-frames={JSON.stringify(frames.image2)}
    data-type="image"
    data-x="right"
    data-y="bottom"
    data-hoffset="['-400','0','0','0']"
    data-voffset="['-310','0','0','0']"
    data-width="['auto']"
    data-height="['auto']"
  >
    <img src="/images/slider/s8.png" alt="layer" width="564" height="560" />
  </div>
</li>
