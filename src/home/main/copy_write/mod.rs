mod render;
pub mod styled;

pub struct CopyWrite<'a> {
    value: &'a str,
}

impl<'a> CopyWrite<'a> {
    pub fn get_value(&self) -> &'a str {
        self.value
    }

    pub fn new() -> CopyWrite<'a> {
        CopyWrite {
            value: "© 2013 Memoize",
        }
    }
}
