use crate::css;

pub fn init() {
    css::push(
        r#"
        .home__main-hero-illustration {
            aspect-ratio: 1 / 1;
            margin: 0 auto;
            max-height: 550px;
            width: 100%;
        }
    "#,
    );
}
