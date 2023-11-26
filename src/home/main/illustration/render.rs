use super::Illustration;
use crate::html;
use std::fmt;

impl<'a> fmt::Display for Illustration<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let template: String = html!(
            <img class="home__main-hero-illustration" src={self.get_src()} alt={self.get_alt()} loading={self.get_loading()} />
        );

        write!(f, "{}", template)
    }
}
