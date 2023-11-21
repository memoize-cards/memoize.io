use super::Home;
use css::style;
use html::*;
use std::fmt;

impl<'a> fmt::Display for Home<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let html_content: String = html!(
            <html lang="pt-BR" translate="no">
                <head>
                    <meta charset="UTF-8">
                    <meta name="description" content={self.description()} />
                    <meta name="theme-color" content="#fafafa" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <title>{self.title()}</title>
                    <link rel="icon" href="//memoize.cards/favicon.ico" sizes="any" />
                    <link rel="icon" href="//memoize.cards/memoize.svg" type="image/svg+xml" />
                    <link rel="apple-touch-icon" href="//memoize.cards/memoize_180w.png" />
                    <link rel="manifest" href="//memoize.cards/manifest.json" />
                    <link rel="preconnect" href="//fonts.gstatic.com" crossorigin />
                    <link rel="preconnect" href="//fonts.googleapis.com" crossorigin />
                    <link rel="preconnect" href="//cdnjs.cloudflare.com" crossorigin />
                    <link rel="stylesheet" href="//fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@48,300,0,0" />
                    <link rel="stylesheet" href="//fonts.googleapis.com/css2?family=Roboto+Condensed:wght@400;500;700&family=Roboto:wght@400;500;700&display=swap" />
                    {style()}
                </head>
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
