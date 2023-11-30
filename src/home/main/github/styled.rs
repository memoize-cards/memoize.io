use crate::css;

pub fn init() {
    css::push(
        r#"
        .home__main-side-github {
            align-items: center;
            border: none;
            cursor: pointer;
            display: flex;
            height: 42px;
            justify-content: center;
            overflow: hidden;
            text-indent: -9999px;
            width: 42px;
        }
    "#,
    );
}
