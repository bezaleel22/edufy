// Global type declarations for jQuery and Revolution Slider

interface JQueryStatic {
  (selector: string): JQuery;
  fn: {
    revolution: (options?: RevolutionOptions) => JQuery;
  };
}

interface JQuery {
  show(): JQuery;
  revolution(options?: RevolutionOptions): JQuery;
  data(key: string): unknown;
}

interface RevolutionOptions {
  delay?: number;
  responsiveLevels?: number[];
  gridwidth?: number[];
  jsFileLocation?: string;
  sliderLayout?: string;
  navigation?: {
    keyboardNavigation?: string;
    keyboard_direction?: string;
    mouseScrollNavigation?: string;
    onHoverStop?: string;
    bullets?: {
      enable?: boolean;
      style?: string;
      hide_onmobile?: boolean;
      hide_under?: number;
      h_align?: string;
      v_align?: string;
      h_offset?: number;
      hide_onleave?: boolean;
      v_offset?: number;
      space?: number;
      tmp?: string;
    };
    arrows?: {
      enable?: boolean;
    };
  };
}

declare global {
  interface Window {
    jQuery: JQueryStatic;
    $: JQueryStatic;
  }
}

export {};
