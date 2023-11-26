use crate::head::Head;
mod handle;
mod render;
pub mod styled;

pub struct Home<'a> {
    head: &'a Head<'a>,
}

impl<'a> Home<'a> {
    pub fn get_head(&self) -> &'a Head<'a> {
        self.head
    }

    pub fn new(head: &'a Head<'a>) -> Home<'a> {
        Home { head }
    }
}
