mod render;
pub mod styled;

pub struct Github<'a> {
    name: &'a str,
    url: &'a str,
}

impl<'a> Github<'a> {
    pub fn get_name(&self) -> &'a str {
        self.name
    }

    pub fn get_url(&self) -> &'a str {
        self.url
    }

    pub fn new() -> Github<'a> {
        Github {
            name: "Github",
            url: "//github.com/memoize-cards",
        }
    }
}
