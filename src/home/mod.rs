use crate::head::Head;
use crate::home::header::Header;
mod handle;
mod header;
mod render;
pub mod styled;

pub struct Home<'a> {
    head: &'a Head<'a>,
    header: &'a Header<'a>,
}

impl<'a> Home<'a> {
    pub fn get_head(&self) -> &'a Head<'a> {
        self.head
    }

    pub fn get_header(&self) -> &'a Header<'a> {
        self.header
    }

    pub fn new(head: &'a Head<'a>, header: &'a Header<'a>) -> Home<'a> {
        Home { head, header }
    }
}
