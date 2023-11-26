use super::Footer;
use crate::html;
use std::fmt;

impl<'a> fmt::Display for Footer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let template: String = html!(
            <footer class="home__footer">
                <div class="home__footer-container">
                    <span class="home__footer-benefit">{self.get_how_it_work()}</span>
                    <span class="home__footer-benefit">{self.get_usage()}</span>
                    <span class="home__footer-benefit">{self.get_benefits()}</span>
                    <span class="home__footer-benefit">{self.get_try_it_out()}</span>
                    {self.get_language()}
                </div>
            </footer>
        );

        write!(f, "{}", template)
    }
}
