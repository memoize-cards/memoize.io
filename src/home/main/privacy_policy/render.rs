use super::PrivacyPolicy;
use crate::html;
use std::fmt;

impl<'a> fmt::Display for PrivacyPolicy<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let template: String = html!(
            <div class="home__main-side-privacy-policy">
                <a href={self.get_url()}>{self.get_text()}</a>
            </div>
        );

        write!(f, "{}", template)
    }
}
