use super::TermsOfUse;
use crate::html;
use std::fmt;

impl<'a> fmt::Display for TermsOfUse<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let template: String = html!(
            <div class="home__main-side-terms-of-use">
                <a href={self.get_url()}>{self.get_text()}</a>
            </div>
        );

        write!(f, "{}", template)
    }
}
