use crate::css::*;

pub fn init() {
    css(r#"
        :root {
            --border-radius-none: 0;
            --border-radius-sm: 8px;
            --border-radius-md: 16px;
            --border-radius-lg: 24px;
            --border-radius-pill: 500px;
            --border-radius-circular: 50%;

            --border-width-none: 0;
            --border-width-hairline: 1px;
            --border-width-thin: 2px;
            --border-width-thick: 4px;
            --border-width-heavy: 8px;

            --color-master-darkest: #0a0a0a;
            --color-master-darker: #1a1a1a;
            --color-master-dark: #2c2c2c;
            --color-master: #626262;
            --color-master-light: #e6e6e6;
            --color-master-lighter: #f0f0f0;
            --color-master-lightest: #fafafa;
            --color-primary-darker: #413768;
            --color-primary-dark: #5b4d91;
            --color-primary: #6d5cae;
            --color-primary-light: #8a7dbe;
            --color-primary-lighter: #e2deef;
            --color-complete-darker: #2b6a94;
            --color-complete-dark: #3c93ce;
            --color-complete: #48b0f7;
            --color-complete-light: #6dc0f9;
            --color-complete-lighter: #daeffd;
            --color-success-darker: #0a7c71;
            --color-success-dark: #0dad9e;
            --color-success: #10cfbd;
            --color-success-light: #40d9ca;
            --color-success-lighter: #cff5f2;
            --color-warning-darker: #957d32;
            --color-warning-dark: #cfae45;
            --color-warning: #f8d053;
            --color-warning-light: #f9d975;
            --color-warning-lighter: #fef6dd;
            --color-danger-darker: #933432;
            --color-danger-dark: #cd4945;
            --color-danger: #f55753;
            --color-danger-light: #f77975;
            --color-danger-lighter: #fddddd;
            --color-info-darker: #232b31;
            --color-info-dark: #313b44;
            --color-info: #3b4752;
            --color-info-light: #626c75;
            --color-info-lighter: #d8dadc;
            --color-menu-dark: #21252d;
            --color-menu: #2b303b;
            --color-menu-light: #929aac;
            --color-pure-white: #fff;
            --color-pure-black: #000;

            --font-family-highlight: 'Roboto Condensed', sans-serif;
            --font-family-base: 'Roboto', sans-serif;

            --font-size-xxxs: 12px;
            --font-size-xxs: 14px;
            --font-size-xs: 16px;
            --font-size-sm: 20px;
            --font-size-md: 24px;
            --font-size-lg: 32px;
            --font-size-xl: 40px;
            --font-size-xxl: 48px;
            --font-size-xxxl: 64px;
            --font-size-display: 80px;
            --font-size-giant: 96px;

            --font-weight-bold: 700;
            --font-weight-medium: 500;
            --font-weight-regular: 400;

            --line-height-default: 100%;
            --line-height-xs: 115%;
            --line-height-sm: 120%;
            --line-height-md: 133%;
            --line-height-lg: 150%;
            --line-height-xl: 170%;
            --line-height-xxl: 200%;

            --opacity-level-semiopaque: 0.72;
            --opacity-level-intense: 0.64;
            --opacity-level-medium: 0.32;
            --opacity-level-light: 0.16;
            --opacity-level-semitransparent: 0.08;

            --shadow-level-1: 0 4px 8px;
            --shadow-level-2: 0 8px 24px;
            --shadow-level-3: 0 16px 32px;
            --shadow-level-4: 0 16px 48px;

            --spacing-quarck: 4px;
            --spacing-nano: 8px;
            --spacing-xxxs: 16px;
            --spacing-xxs: 24px;
            --spacing-xs: 32px;
            --spacing-sm: 40px;
            --spacing-md: 48px;
            --spacing-lg: 56px;
            --spacing-xl: 64px;
            --spacing-xxl: 80px;
            --spacing-xxxl: 120px;
            --spacing-huge: 160px;
            --spacing-giant: 200px;
            --spacing_inset-quarck: 4px;
            --spacing_inset-nano: 8px;
            --spacing_inset-xs: 16px;
            --spacing_inset-sm: 24px;
            --spacing_inset-md: 32px;
            --spacing_inset-lg: 40px;
            --spacing_inset-huge: 48px;
            --spacing_inset-giant: 56px;
        }
    "#);
}
