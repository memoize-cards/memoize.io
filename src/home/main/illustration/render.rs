use super::Illustration;
use crate::html;
use std::fmt;

impl fmt::Display for Illustration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let template: String = html!(
            <div class="home__main-hero-illustration">
                {include_str!("./media.svg")}
            </div>
        );

        write!(f, "{}", template)
    }
}
