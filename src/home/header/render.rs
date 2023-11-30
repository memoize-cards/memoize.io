use super::Header;
use crate::html;
use std::fmt;

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let template: String = html!(
            <header class="home__header">
                {include_str!("./media.svg")}
            </header>
        );

        write!(f, "{}", template)
    }
}
