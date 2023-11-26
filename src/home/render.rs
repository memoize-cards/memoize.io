use super::Home;
use crate::html;
use std::fmt;

impl<'a> fmt::Display for Home<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let template: String = html!(
            <html lang="pt-BR" translate="no">
                {self.get_head()}
                <body>
                    {self.get_header()}
                    {self.get_main()}
                    <footer class="home__footer">
                        <div class="home__footer-container">
                            <span class="home__footer-benefit">{"Organize informações facilmente usando cartões de estudo"}</span>
                            <span class="home__footer-benefit">{"Revisite e reforce seu conhecimento com a repetição espaçada"}</span>
                            <span class="home__footer-benefit">{"Domine novas palavras, revise conceitos, prepare-se para exames e reforce seu aprendizado"}</span>
                            <span class="home__footer-benefit">{"Simplicidade, foco no essencial e aprendizado em qualquer lugar"}</span>
                            <label class="home__footer-language">
                                <span class="home__footer-language-icon material-symbols-outlined">language</span>
                                <select class="home__footer-language-select">
                                    <option class="home__footer-language-option" value="en-US">EN</option>
                                    <option class="home__footer-language-option" value="es">ES</option>
                                    <option class="home__footer-language-option" value="pt-BR" selected>PT</option>
                                </select>
                            </label>
                        </div>
                    </footer>
                </body>
            </html>
        );

        write!(f, "{}", template)
    }
}
