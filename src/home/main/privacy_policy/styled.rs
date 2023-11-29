use crate::css;

pub fn init() {
    css::push(
        r#"
        .home__main-side-privacy-policy {
            align-items: center;
            display: flex;
            justify-content: center;
        }

        .home__main-side-privacy-policy a {
            color: var(--color-primary);
            display: flex;
            font-family: var(--font-family-base);
            font-size: var(--font-size-xxxs);
            font-weight: var(--font-weight-regular);
            line-height: var(--line-height-xl);
            text-decoration: none;
            text-orientation: sideways;
            writing-mode: vertical-lr;
        }
    "#,
    );
}
