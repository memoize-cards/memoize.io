use css::*;

pub fn init() {
    css(r#"
        *,
        *::after,
        *::before {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }

        body {
            background-color: var(--color-master-lightest);
        }
    "#);
}