use super::Header;
use crate::html;
use std::fmt;

impl<'a> fmt::Display for Header<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let template: String = html!(
            <header class="home__header">
                <img class="home__header-logo" src={self.get_src()} alt={self.get_alt()} loading={self.get_loading()} />
            </header>
        );

        write!(f, "{}", template)
    }
}
