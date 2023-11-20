mod handle;
mod template;

pub struct Home<'a> {
    description: &'a str,
    title: &'a str,
}

impl<'a> Home<'a> {
    pub fn description(&self) -> &'a str {
        self.description
    }

    pub fn title(&self) -> &'a str {
        self.title
    }
}
