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
    "#);
}
