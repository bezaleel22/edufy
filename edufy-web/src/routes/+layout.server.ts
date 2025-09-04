import type { LayoutServerLoad } from './$types';
import { SITE_TITLE, SITE_DESCRIPTION, SITE_URL, SITE_IMAGE } from '$lib/constants';

export const load: LayoutServerLoad = async ({ url }) => {
  // Default SEO data
  let title = SITE_TITLE;
  let description = SITE_DESCRIPTION;
  let keywords = 'education, school, Nigeria, academy, nursery, primary, secondary, quality education';
  
  // Page-specific SEO
  switch (url.pathname) {
    case '/':
      title = 'Lighthouse Leading Academy - Premier Educational Institution in Nigeria';
      description = 'Lighthouse Leading Academy offers quality education from nursery to secondary level with modern facilities, experienced teachers, and a commitment to academic excellence in Nigeria.';
      keywords = 'lighthouse academy, education Nigeria, private school, nursery school, primary school, secondary school, quality education, academic excellence';
      break;
    case '/about':
      title = 'About Us - Lighthouse Leading Academy';
      description = 'Learn about Lighthouse Leading Academy\'s mission, vision, and commitment to providing quality education. Meet our experienced faculty and discover our modern facilities.';
      keywords = 'about lighthouse academy, school mission, educational philosophy, experienced teachers, modern facilities';
      break;
    case '/admission':
      title = 'Admission - Lighthouse Leading Academy';
      description = 'Apply for admission to Lighthouse Leading Academy. Learn about our admission process, requirements, and how to join our community of academic excellence.';
      keywords = 'lighthouse academy admission, school admission Nigeria, apply to school, admission requirements, school enrollment';
      break;
    case '/portfolio':
      title = 'Portfolio - Lighthouse Leading Academy';
      description = 'Explore Lighthouse Leading Academy\'s portfolio showcasing student achievements, school activities, facilities, and our commitment to educational excellence.';
      keywords = 'lighthouse academy portfolio, school activities, student achievements, school facilities, educational programs';
      break;
    case '/blog':
      title = 'Blog - Lighthouse Leading Academy';
      description = 'Read the latest news, updates, and educational insights from Lighthouse Leading Academy. Stay informed about school events, academic achievements, and educational tips.';
      keywords = 'lighthouse academy blog, school news, educational insights, school events, academic updates';
      break;
    case '/contact':
      title = 'Contact Us - Lighthouse Leading Academy';
      description = 'Get in touch with Lighthouse Leading Academy. Find our contact information, location, and how to reach us for inquiries about admissions and school programs.';
      keywords = 'contact lighthouse academy, school contact, school location, phone number, email address';
      break;
  }

  const currentUrl = `${SITE_URL}${url.pathname}`;
  
  return {
    seo: {
      title,
      description,
      keywords,
      url: currentUrl,
      image: SITE_IMAGE,
      type: 'website'
    }
  };
};
