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
                    <div class="home__main-install-icon">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960" height="24" width="24" fill="currentColor">
                            <path d="M280-40q-33 0-56.5-23.5T200-120v-720q0-33 23.5-56.5T280-920h280v80H280v40h280v80H280v480h400v-80h80v200q0 33-23.5 56.5T680-40H280Zm0-120v40h400v-40H280Zm440-240L520-600l56-56 104 104v-288h80v288l104-104 56 56-200 200ZM280-800v-40 40Zm0 640v40-40Z"/>
                        </svg>
                    </div>
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
