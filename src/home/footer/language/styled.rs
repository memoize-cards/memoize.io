use crate::css;

pub fn init() {
    css::push(
        r#"
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
    "#,
    );
}
