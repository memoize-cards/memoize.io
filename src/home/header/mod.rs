use crate::assets;
use crate::loading::Loading;
mod render;
pub mod styled;

pub struct Header<'a> {
    alt: &'a str,
    path: &'a str,
    loading: Loading,
}

impl<'a> Header<'a> {
    pub fn get_alt(&self) -> &'a str {
        self.alt
    }

    pub fn get_src(&self) -> String {
        assets::url_for(self.path)
    }

    pub fn get_loading(&self) -> &str {
        self.loading.as_str()
    }

    pub fn new() -> Header<'a> {
        Header {
            alt: "Memoize",
            path: "memoize.svg",
            loading: Loading::Eager,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_alt() {
        let header = Header::new();
        assert_eq!(header.get_alt(), "Memoize");
    }

    #[test]
    fn get_src() {
        let header = Header::new();
        assert_eq!(header.get_src(), "//assets.memoize.cards/memoize.svg");
    }

    #[test]
    fn get_loading() {
        let header = Header::new();
        assert_eq!(header.get_loading(), "eager");
    }
}
