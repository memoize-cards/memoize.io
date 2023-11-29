use crate::css;

pub fn init() {
    css::push(
        r#"
        .home__main-side-github {
            align-items: center;
            background: url('//assets.memoize.cards/social-media/github.svg');
            background-color: transparent;
            background-position: center;
            background-repeat: no-repeat;
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
