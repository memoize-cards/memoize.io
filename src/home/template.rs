use super::Home;
use html::*;
use std::fmt;

impl<'a> fmt::Display for Home<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let html_content: String = html!(
            <html lang="pt-BR" translate="no">
                {self.head()}
                <body>
                    <header class="home__header">
                        <img class="home__header-logo" src="//memoize.cards/memoize.svg" alt="Memoize" loading="eager" />
                    </header>
                    <main class="home__main">
                        <div class="home__hero">
                            <img class="home__hero-illustration" src="//memoize.cards/media.2e9bcf265f36ffda7cfcebbdbc8c3672.svg" alt="Memoize" loading="eager" />
                            <hgroup class="home__hero-hgroup">
                                <h1 class="home__hero-title">{"Aprenda, Memorize, Domine!"}</h1>
                                <h2 class="home__hero-description">{"Um aplicativo avançado de flashcard projetado para otimizar o processo de aprendizado e memorização."}</h2>
                            </hgroup>
                        </div>
                        <button class="home__hero-install">
                            <span class="home__hero-install-icon material-symbols-outlined">install_mobile</span>
                            <span class="home__hero-install-text">{"Instalar aplicativo"}</span>
                        </button>
                    </main>
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
        write!(f, "{}", html_content)
    }
}
