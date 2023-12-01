use super::language;
use crate::css;

pub fn init() {
    language::styled::init();
    css::push(include_str!("./style.css"));
}
