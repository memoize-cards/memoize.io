use crate::css;

pub fn init() {
    css::push(
        r#"
        .home__header {
            display: flex;
            height: 74px;
            justify-content: space-between;
            margin: 0 auto;
            max-width: 1440px;
            padding: var(--spacing_inset-xs);
            width: 100%;
        }

        .home__header svg {
            aspect-ratio: 1 / 1;
            width: 42px;
        }
    "#,
    );
}
