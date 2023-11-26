use super::Language;
use crate::html;
use std::fmt;

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let template: String = html!(
            <label class="home__footer-language">
                <span class="home__footer-language-icon material-symbols-outlined">language</span>
                <select class="home__footer-language-select">
                    <option class="home__footer-language-option" value="en-US">EN</option>
                    <option class="home__footer-language-option" value="es">ES</option>
                    <option class="home__footer-language-option" value="pt-BR" selected>PT</option>
                </select>
            </label>
        );

        write!(f, "{}", template)
    }
}
