mod render;
pub mod styled;

pub struct PrivacyPolicy<'a> {
    text: &'a str,
    url: &'a str,
}

impl<'a> PrivacyPolicy<'a> {
    pub fn get_text(&self) -> &'a str {
        self.text
    }

    pub fn get_url(&self) -> &'a str {
        self.url
    }

    pub fn new() -> PrivacyPolicy<'a> {
        PrivacyPolicy {
            text: "Política de privacidade",
            url: "io.memoize.cards/politica-de-privacidade",
        }
    }
}
