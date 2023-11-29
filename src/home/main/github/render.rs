use super::Github;
use crate::html;
use std::fmt;

impl<'a> fmt::Display for Github<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let template: String = html!(
            <a class="home__main-side-github" href={self.get_url()} title={self.get_name()}>{self.get_name()}</a>
        );

        write!(f, "{}", template)
    }
}
