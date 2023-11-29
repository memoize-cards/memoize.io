use super::CopyWrite;
use crate::html;
use std::fmt;

impl<'a> fmt::Display for CopyWrite<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let template: String = html!(
            <div class="home__main-side-copy-write">
                <span>{self.get_value()}</span>
            </div>
        );

        write!(f, "{}", template)
    }
}
