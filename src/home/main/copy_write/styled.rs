use crate::css;

pub fn init() {
    css::push(
        r#"
        .home__main-side-copy-write {
            align-items: center;
            display: flex;
            justify-content: center;
            padding-bottom: var(--spacing_inset-lg);
        }

        .home__main-side-copy-write span {
            color: var(--color-master);
            display: flex;
            font-family: var(--font-family-base);
            font-size: var(--font-size-xxxs);
            font-weight: var(--font-weight-regular);
            line-height: var(--line-height-xl);
            text-orientation: sideways;
            writing-mode: vertical-lr;
        }
    "#,
    );
}
