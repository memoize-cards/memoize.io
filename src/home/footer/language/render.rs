use super::Language;
use crate::html;
use std::fmt;

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let template: String = html!(
            <div class="home__footer-language">
                <div class="home__footer-language-icon">
                    {include_str!("./media.svg")}
                </div>
                <select class="home__footer-language-select" aria-label="language">
                    <option class="home__footer-language-option" value="en-US">EN</option>
                    <option class="home__footer-language-option" value="es">ES</option>
                    <option class="home__footer-language-option" value="pt-BR" selected>PT</option>
                </select>
            </div>
        );

        write!(f, "{}", template)
    }
}
