pub enum Loading {
    Auto,
    Eager,
    Lazy,
}

impl Loading {
    pub fn as_str(&self) -> &str {
        match self {
            Loading::Auto => "auto",
            Loading::Eager => "eager",
            Loading::Lazy => "lazy",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto() {
        let loading = Loading::Auto;
        assert_eq!(loading.as_str(), "auto");
    }

    #[test]
    fn eager() {
        let loading = Loading::Eager;
        assert_eq!(loading.as_str(), "eager");
    }

    #[test]
    fn lazy() {
        let loading = Loading::Lazy;
        assert_eq!(loading.as_str(), "lazy");
    }
}
