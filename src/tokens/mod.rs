use crate::css;

pub fn init() {
    css::push(include_str!("./borderRadius.css"));
    css::push(include_str!("./borderWidth.css"));
    css::push(include_str!("./color.css"));
    css::push(include_str!("./fontFamily.css"));
    css::push(include_str!("./fontSize.css"));
    css::push(include_str!("./fontWeight.css"));
    css::push(include_str!("./lineHeight.css"));
    css::push(include_str!("./opacityLevel.css"));
    css::push(include_str!("./shadowLevel.css"));
    css::push(include_str!("./spacing.css"));
}
