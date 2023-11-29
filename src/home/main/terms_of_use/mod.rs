mod render;
pub mod styled;

pub struct TermsOfUse<'a> {
    text: &'a str,
    url: &'a str,
}

impl<'a> TermsOfUse<'a> {
    pub fn get_text(&self) -> &'a str {
        self.text
    }

    pub fn get_url(&self) -> &'a str {
        self.url
    }

    pub fn new() -> TermsOfUse<'a> {
        TermsOfUse {
            text: "Termos de uso",
            url: "io.memoize.cards/termos-de-uso",
        }
    }
}
