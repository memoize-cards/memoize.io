use crate::home::footer::Footer;
use crate::home::header::Header;
use crate::home::main::Main;
mod footer;
mod handle;
mod header;
mod main;
mod render;
pub mod styled;

pub struct Home<'a> {
    header: Header,
    main: Main<'a>,
    footer: Footer<'a>,
}

impl<'a> Home<'a> {
    pub fn get_footer(&self) -> &Footer<'a> {
        &self.footer
    }

    pub fn get_header(&self) -> &Header {
        &self.header
    }

    pub fn get_main(&self) -> &Main<'a> {
        &self.main
    }

    pub fn new() -> Home<'a> {
        let header = Header::new();
        let main = Main::new();
        let footer = Footer::new();
        Home {
            header,
            main,
            footer,
        }
    }
}
