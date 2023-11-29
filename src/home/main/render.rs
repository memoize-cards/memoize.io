use super::Main;
use crate::html;
use std::fmt;

impl<'a> fmt::Display for Main<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let template: String = html!(
            <main class="home__main">
                <div class="home__main-hero">
                    {self.get_illustration()}
                    <hgroup class="home__main-hero-hgroup">
                        <h1 class="home__main-hero-title">{self.get_title()}</h1>
                        <h2 class="home__main-hero-description">{self.get_description()}</h2>
                    </hgroup>
                </div>
                <button class="home__main-install">
                    <span class="home__main-install-icon material-symbols-outlined">install_mobile</span>
                    <span class="home__main-install-text">{"Instalar aplicativo"}</span>
                </button>
                <div class="home__main-side">
                    {self.get_privacy_policy()}
                    {self.get_terms_of_use()}
                    {self.get_copy_write()}
                    {self.get_linkedin()}
                    {self.get_github()}
                </div>
            </main>
        );

        write!(f, "{}", template)
    }
}
