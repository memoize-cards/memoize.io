use super::LinkedIn;
use crate::html;
use std::fmt;

impl<'a> fmt::Display for LinkedIn<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let template: String = html!(
            <a class="home__main-side-linkedin" href={self.get_url()} title={self.get_name()}>{self.get_name()}</a>
        );

        write!(f, "{}", template)
    }
}
