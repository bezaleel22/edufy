<script lang="ts">
  import { page } from "$app/stores";
  import "$lib/assets";
  import Footer from "$lib/components/Footer.svelte";
  import Header from "$lib/components/Header.svelte";
  import PageHeader from "$lib/components/PageHeader.svelte";
  import { SITE_URL, SITE_IMAGE } from "$lib/constants";
  import type { LayoutData } from "./$types";

  import { loading } from "$lib/stores";
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";

  export let data: LayoutData;

  onMount(() => {
    $loading = document.readyState === "loading";
    if ($loading) {
      document.addEventListener("DOMContentLoaded", () => ($loading = false));
    }
  });

  $: seo = (data as any)?.seo || {};
</script>

<svelte:head>
  <!-- Primary Meta Tags -->
  <title>{seo.title || "Lighthouse Leading Academy"}</title>
  <meta name="title" content={seo.title || "Lighthouse Leading Academy"} />
  <meta
    name="description"
    content={seo.description || "Premier educational institution in Nigeria"}
  />
  <meta
    name="keywords"
    content={seo.keywords || "education, school, Nigeria"}
  />
  <meta name="author" content="Lighthouse Leading Academy" />
  <meta name="robots" content="index, follow" />
  <meta name="language" content="English" />
  <meta name="revisit-after" content="7 days" />

  <!-- Canonical URL -->
  <link rel="canonical" href={seo.url || SITE_URL} />

  <!-- Open Graph / Facebook -->
  <meta property="og:type" content={seo.type || "website"} />
  <meta property="og:url" content={seo.url || SITE_URL} />
  <meta
    property="og:title"
    content={seo.title || "Lighthouse Leading Academy"}
  />
  <meta
    property="og:description"
    content={seo.description || "Premier educational institution in Nigeria"}
  />
  <meta property="og:image" content={seo.image || SITE_IMAGE} />
  <meta property="og:site_name" content="Lighthouse Leading Academy" />
  <meta property="og:locale" content="en_US" />

  <!-- Twitter -->
  <meta property="twitter:card" content="summary_large_image" />
  <meta property="twitter:url" content={seo.url || SITE_URL} />
  <meta
    property="twitter:title"
    content={seo.title || "Lighthouse Leading Academy"}
  />
  <meta
    property="twitter:description"
    content={seo.description || "Premier educational institution in Nigeria"}
  />
  <meta property="twitter:image" content={seo.image || SITE_IMAGE} />

  <!-- Additional SEO -->
  <meta name="theme-color" content="#1a73e8" />
  <meta name="msapplication-TileColor" content="#1a73e8" />

  <!-- Structured Data -->
  {@html `<script type="application/ld+json">
    {
      "@context": "https://schema.org",
      "@type": "EducationalOrganization",
      "name": "Lighthouse Leading Academy",
      "alternateName": "LL Academy",
      "description": "${seo.description || "Premier educational institution in Nigeria"}",
      "url": "${SITE_URL}",
      "logo": "${SITE_IMAGE}",
      "image": "${seo.image || SITE_IMAGE}",
      "foundingDate": "2010",
      "address": {
        "@type": "PostalAddress",
        "addressCountry": "Nigeria",
        "addressRegion": "Lagos State"
      },
      "contactPoint": [
        {
          "@type": "ContactPoint",
          "telephone": "+2348127823406",
          "contactType": "customer service",
          "availableLanguage": "English"
        },
        {
          "@type": "ContactPoint",
          "telephone": "+2349169801738",
          "contactType": "admissions",
          "availableLanguage": "English"
        }
      ],
      "offers": {
        "@type": "EducationalOccupationalProgram",
        "name": "Quality Education Programs",
        "description": "Nursery, Primary, and Secondary Education",
        "provider": {
          "@type": "EducationalOrganization",
          "name": "Lighthouse Leading Academy"
        }
      },
      "sameAs": [
        "https://facebook.com/lighthouseleadingacademy",
        "https://twitter.com/llacademy"
      ]
    }
  </script>`}
</svelte:head>

<!-- <ScriptLoader /> -->
{#if $loading}
  <div class="preloader clock text-center" out:fade={{ duration: 800 }}>
    <div class="queraLoader" out:fade={{ duration: 500 }}>
      <div class="loaderO">
        <span>L</span>
        <span>I</span>
        <span>G</span>
        <span>H</span>
        <span>T</span>
        <span>H</span>
        <span>O</span>
        <span>U</span>
        <span>S</span>
        <span>E</span>
      </div>
    </div>
  </div>
{/if}

{#if $page.url.pathname === "/"}
  <Header />
{:else}
  <PageHeader />
{/if}

<main class="app">
  <slot />
</main>

<Footer />
