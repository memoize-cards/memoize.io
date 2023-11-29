mod render;
pub mod styled;

pub struct LinkedIn<'a> {
    name: &'a str,
    url: &'a str,
}

impl<'a> LinkedIn<'a> {
    pub fn get_name(&self) -> &'a str {
        self.name
    }

    pub fn get_url(&self) -> &'a str {
        self.url
    }

    pub fn new() -> LinkedIn<'a> {
        LinkedIn {
            name: "LinkedIn",
            url: "//www.linkedin.com/company/memoize",
        }
    }
}
