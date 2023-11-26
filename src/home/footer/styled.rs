use super::language;
use crate::css;

pub fn init() {
    language::styled::init();
    css::push(
        r#"
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
            line-height: var(--line-height-xl);
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
    "#,
    );
}
