use crate::css;

pub fn init() {
    css::push(
        r#"
        .home__main-hero-illustration {
            align-items: center;
            aspect-ratio: 1 / 1;
            display: flex;
            justify-content: center;
            margin: 0 auto;
            max-height: 550px;
            width: 100%;
        }

        .home__main-hero-illustration svg {
            max-height: inherit;
        }
    "#,
    );
}
