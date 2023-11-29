use super::github;
use super::illustration;
use super::linkedin;
use crate::css;

pub fn init() {
    github::styled::init();
    illustration::styled::init();
    linkedin::styled::init();
    css::push(
        r#"
        .home__main {
            align-items: start;
            display: flex;
            flex-direction: column;
            gap: var(--spacing_inset-lg);
            height: calc(100svh - 148px);
            margin: 0 auto;
            max-width: 1440px;
            padding: var(--spacing_inset-xs);
            position: relative;
            width: 100%;
        }

        .home__main-hero {
            align-items: start;
            display: flex;
            flex-direction: column;
            gap: var(--spacing_inset-md);
            height: calc(100svh - 180px);
            justify-content: end;
            width: calc(100% - 42px);
        }

        .home__main-hero-hgroup {
            display: flex;
            flex-direction: column;
            gap: var(--spacing_inset-nano);
            max-width: 327px;
        }

        .home__main-hero-title {
            color: var(--color-master-dark);
            display: flex;
            font-family: var(--font-family-highlight);
            font-size: var(--font-size-lg);
            font-weight: var(--font-weight-bold);
            line-height: var(--line-height-sm);
        }

        .home__main-hero-description {
            color: var(--color-master);
            display: flex;
            font-family: var(--font-family-base);
            font-size: var(--font-size-xxxs);
            font-weight: var(--font-weight-regular);
            line-height: var(--line-height-xl);
        }

        .home__main-install {
            align-items: center;
            background-color: transparent;
            border: var(--border-width-hairline) solid var(--color-primary);
            border-radius: var(--border-radius-sm);
            cursor: pointer;
            display: flex;
            gap: var(--spacing_inset-nano);
            padding: var(--spacing_inset-nano);
            padding-right: var(--spacing_inset-xs);
        }

        .home__main-install-icon {
            color: var(--color-primary);
            font-size: 24px;
        }

        .home__main-install-text {
            color: var(--color-primary);
            display: flex;
            font-family: var(--font-family-base);
            font-size: var(--font-size-xxs);
            font-weight: var(--font-weight-medium);
        }

        .home__main-side {
            display: flex;
            flex-direction: column;
            gap: var(--spacing_inset-xs);
            height: calc(100svh - 180px);
            justify-content: end;
            position: absolute;
            right: 16px;
            top: 16px;
            width: 42px;
        }
    "#,
    );
}
