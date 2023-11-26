use crate::assets;
use crate::loading::Loading;
mod render;
pub mod styled;

pub struct Illustration<'a> {
    alt: &'a str,
    path: &'a str,
    loading: Loading,
}

impl<'a> Illustration<'a> {
    pub fn get_alt(&self) -> &'a str {
        self.alt
    }

    pub fn get_src(&self) -> String {
        assets::url_for(self.path)
    }

    pub fn get_loading(&self) -> &str {
        self.loading.as_str()
    }

    pub fn new() -> Illustration<'a> {
        Illustration {
            alt: "Memoize",
            path: "illustration/home.svg",
            loading: Loading::Eager,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_alt() {
        let header = Illustration::new();
        assert_eq!(header.get_alt(), "Memoize");
    }

    #[test]
    fn get_src() {
        let header = Illustration::new();
        assert_eq!(
            header.get_src(),
            "//memoize.cards/media.2e9bcf265f36ffda7cfcebbdbc8c3672.svg"
        );
    }

    #[test]
    fn get_loading() {
        let header = Illustration::new();
        assert_eq!(header.get_loading(), "eager");
    }
}
