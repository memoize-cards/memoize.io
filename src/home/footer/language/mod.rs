use crate::assets;
mod render;
pub mod styled;

pub struct Language<'a> {
    url: &'a str,
}

impl<'a> Language<'a> {
    pub fn get_url(&self) -> String {
        assets::url_for(self.url)
    }

    pub fn new() -> Language<'a> {
        Language {
            url: "icone/language.svg",
        }
    }
}
