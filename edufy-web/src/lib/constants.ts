// if (browser) console.log(dev)

import { dev } from "$app/environment";

// These settings will be use to generate your SEO tags
export const SITE_TITLE =
  "Lighthouse Leading Academy - Premier Educational Institution in Nigeria";
export const SITE_DESCRIPTION =
  "Lighthouse Leading Academy is a premier educational institution in Nigeria, providing quality education from nursery to secondary level with modern facilities and experienced teachers.";
export const SITE_LOGO = "https://edufy-905.pages.dev/images/logo2.png";
export const SITE_IMAGE = "https://edufy-905.pages.dev/images/logo2.png";
export const SITE_URL = "https://edufy-905.pages.dev";
export const SHOPAPI_URL = !dev
  ? "http://localhost:3000/shop-api"
  : "https://store.beznet.org/shop-api";
export const PAYSTACK_URL = "https://api.paystack.co/transaction";
export const PAYSTACK_PUBLIC_KEY =
  "pk_test_3b604e0ff89da154ea5a401c120607281e714724";

export const AUTH_TOKEN = "auth_token";
export const REQUIRE_VERIFICATION = true;
export const DEFAULT_CURRENCY = "NGN";
export const DEFAULT_LOCALE = "en";
export const CUSTOMER_NOT_DEFINED_ID = "Guest";
export const IMAGE_RESOLUTIONS = [1000, 800, 600, 400];

// To enable local pickup as a delivery option, enter the code of the Vendure shipping method that will be used for local pickup
export const LOCAL_PICKUP = "local-pickup";
export const PUBLIC_LOCAL_PICKUP_CODE = "";
