use css::*;

pub fn init() {
    css(r#"
        .home__header {
            display: flex;
            height: 74px;
            justify-content: space-between;
            margin: 0 auto;
            max-width: 1440px;
            padding: var(--spacing_inset-xs);
            width: 100%;
        }

        .home__header-logo {
            aspect-ratio: 1 / 1;
            width: 42px;
        }

        .home__main {
            display: flex;
            flex-direction: column;
            gap: var(--spacing_inset-lg);
            margin: 0 auto;
            max-width: 1440px;
            min-height: calc(100svh - 148px);
            padding: var(--spacing_inset-xs);
            position: relative;
            width: 100%;
        }

        .home__hero {
            align-items: start;
            display: flex;
            flex-direction: column;
            gap: var(--spacing_inset-md);
            height: calc(100svh - 180px);
            justify-content: end;
            width: calc(100% - 42px);
        }

        .home__hero-illustration {
            aspect-ratio: 1 / 1;
            margin: 0 auto;
            max-height: 550px;
            width: 100%;
        }

        .home__hero-hgroup {
            max-width: 327px;
        }

        .home__hero-h1 {
            color: var(--color-master-dark);
            display: flex;
            font-family: var(--font-family-highlight);
            font-size: var(--font-size-lg);
            font-weight: var(--font-weight-bold);
            line-height: var(--line-height-md);
        }

        .home__footer {
            background-color: var(--color-pure-white);
            height: 74px;
            margin: 0 auto;
            width: 100%;
        }

        .home__footer-container {
            align-items: center;
            display: flex;
            gap: var(--spacing_inset-lg);
            height: inherit;
            justify-content: space-between;
            margin: 0 auto;
            max-width: 1440px;
            padding: var(--spacing_inset-xs);
        }

        .home__footer-benefit {
            color: var(--color-master);
            display: flex;
            font-family: var(--font-family-base);
            font-size: var(--font-size-xxxs);
            font-weight: var(--font-weight-regular);
            line-height: var(--line-height-lg);
        }

        @media (width <= 768px) {
            .home__footer-benefit:nth-child(4n+2) {
                display: none;
            }
        }

        @media (width >= 769px) {
            .home__footer-benefit {
                width: 50%;
            }
        }

        @media (width <= 1024px) {
            .home__footer-benefit:nth-child(4n+3) {
                display: none;
            }
        }

        @media (width >= 1025px) {
            .home__footer-benefit {
                width: 33.33%;
            }
        }

        @media (width <= 1440px) {
            .home__footer-benefit:nth-child(4n+4) {
                display: none;
            }
        }

        @media (width >= 1441px) {
            .home__footer-benefit {
                width: 25%;
            }
        }

        .home__footer-language {
            align-items: center;
            background-color: transparent;
            border: none;
            cursor: pointer;
            display: flex;
            height: 42px;
            justify-content: center;
            min-width: 42px;
            position: relative;
            width: 42px;
        }

        .home__footer-language-icon {
            color: var(--color-primary);
            font-size: 24px;
        }

        .home__footer-language-select {
            appearance: none;
            background-color: transparent;
            border: none;
            color: transparent;
            cursor: pointer;
            height: 42px;
            left: 0;
            position: absolute;
            top: 0;
            width: 42px;
        }

        .home__footer-language-select:focus {
            outline: none;
        }

        .home__footer-language-option {
            color: var(--color-master-dark);
        }
    "#);
}
