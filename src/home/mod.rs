use crate::head::Head;
use crate::home::header::Header;
use crate::home::main::Main;
mod handle;
mod header;
mod main;
mod render;
pub mod styled;

pub struct Home<'a> {
    head: Head<'a>,
    header: Header<'a>,
    main: Main<'a>,
}

impl<'a> Home<'a> {
    pub fn get_head(&self) -> &Head<'a> {
        &self.head
    }

    pub fn get_header(&self) -> &Header<'a> {
        &self.header
    }

    pub fn get_main(&self) -> &Main<'a> {
        &self.main
    }

    pub fn new() -> Home<'a> {
        let head = Head {
            description: "Aprenda, Memorize, Domine!",
            title: "Memoize",
        };
        let header = Header::new();
        let main = Main::new();
        Home { head, header, main }
    }
}
