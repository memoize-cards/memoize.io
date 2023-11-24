use crate::head::Head;
mod handle;
pub mod style;
mod template;

pub struct Home<'a> {
    head: &'a Head<'a>,
}

impl<'a> Home<'a> {
    pub fn head(&self) -> &'a Head {
        self.head
    }
}
