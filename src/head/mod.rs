mod template;

pub struct Head<'a> {
    pub description: &'a str,
    pub title: &'a str,
}

impl<'a> Head<'a> {
    pub fn get_description(&self) -> &'a str {
        self.description
    }

    pub fn get_title(&self) -> &'a str {
        self.title
    }
}
