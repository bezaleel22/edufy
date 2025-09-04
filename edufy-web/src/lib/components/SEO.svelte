<script lang="ts">
  import { page } from "$app/stores";
  import {
    SITE_TITLE,
    SITE_DESCRIPTION,
    SITE_URL,
    SITE_IMAGE,
  } from "$lib/constants";

  export let title: string = SITE_TITLE;
  export let description: string = SITE_DESCRIPTION;
  export let image: string = SITE_IMAGE;
  export let type: string = "website";
  export let keywords: string =
    "education, school, Nigeria, academy, nursery, primary, secondary, quality education";
  export let author: string = "Lighthouse Leading Academy";
  export let canonical: string = "";

  $: currentUrl = canonical || `${SITE_URL}${$page.url.pathname}`;
  $: pageTitle =
    title === SITE_TITLE ? title : `${title} | Lighthouse Leading Academy`;
</script>

<svelte:head>
  <!-- Primary Meta Tags -->
  <title>{pageTitle}</title>
  <meta name="title" content={pageTitle} />
  <meta name="description" content={description} />
  <meta name="keywords" content={keywords} />
  <meta name="author" content={author} />
  <meta name="robots" content="index, follow" />
  <meta name="language" content="English" />
  <meta name="revisit-after" content="7 days" />

  <!-- Canonical URL -->
  <link rel="canonical" href={currentUrl} />

  <!-- Open Graph / Facebook -->
  <meta property="og:type" content={type} />
  <meta property="og:url" content={currentUrl} />
  <meta property="og:title" content={pageTitle} />
  <meta property="og:description" content={description} />
  <meta property="og:image" content={image} />
  <meta property="og:site_name" content="Lighthouse Leading Academy" />
  <meta property="og:locale" content="en_US" />

  <!-- Twitter -->
  <meta property="twitter:card" content="summary_large_image" />
  <meta property="twitter:url" content={currentUrl} />
  <meta property="twitter:title" content={pageTitle} />
  <meta property="twitter:description" content={description} />
  <meta property="twitter:image" content={image} />

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
      "description": "${description}",
      "url": "${SITE_URL}",
      "logo": "${SITE_IMAGE}",
      "image": "${image}",
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
