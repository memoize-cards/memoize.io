#[macro_export]
macro_rules! css {
    ($selector:ident $($look_ahead:tt)*) => {
        format!("{}{}", stringify!($selector), css!($($look_ahead)*))
    };

    (* $($look_ahead:tt)*) => {
        format!("*{}", css!($($look_ahead)*))
    };

    (# $($look_ahead:tt)*) => {
        format!("#{}", css!($($look_ahead)*))
    };

    (. $($look_ahead:tt)*) => {
        format!(".{}", css!($($look_ahead)*))
    };

    (> $($look_ahead:tt)*) => {
        format!(" > {}", css!($($look_ahead)*))
    };

    (+ $($look_ahead:tt)*) => {
        format!(" + {}", css!($($look_ahead)*))
    };

    (~ $($look_ahead:tt)*) => {
        format!(" ~ {}", css!($($look_ahead)*))
    };

    (, $($look_ahead:tt)*) => {
        format!(", {}", css!($($look_ahead)*))
    };

    (: $($look_ahead:tt)*) => {
        format!(":{}", css!($($look_ahead)*))
    };

    (:: $($look_ahead:tt)*) => {
        format!("::{}", css!($($look_ahead)*))
    };

    ({ $($property:tt:$value:tt;)* } $($look_ahead:tt)*) => { "" };

    () => { "" }
}

#[cfg(test)]
mod tests {
    #[test]
    fn universal_selector() {
        let style: String = css!(*);
        assert_eq!(style, "*")
    }

    #[test]
    fn element_selector() {
        let style: String = css!(h1);
        assert_eq!(style, "h1");
    }

    #[test]
    fn id_selector() {
        let style: String = css!(#introduction);
        assert_eq!(style, "#introduction");
    }

    #[test]
    fn class_selector() {
        let style: String = css!(.note);
        assert_eq!(style, ".note");
    }

    #[test]
    fn child_selector() {
        let style: String = css!(li > a);
        assert_eq!(style, "li > a");
    }

    #[test]
    fn descendant_selector() {
        let style: String = css!(p .hidden span);
        assert_eq!(style, "p .hidden span");
    }

    #[test]
    fn adjacent_selector() {
        let style: String = css!(h1 + p);
        assert_eq!(style, "h1 + p");
    }

    #[test]
    fn brother_selector() {
        let style: String = css!(h1 ~ p);
        assert_eq!(style, "h1 ~ p");
    }

    #[test]
    fn grouping_selector() {
        let style: String = css!(h1, h2, h3);
        assert_eq!(style, "h1, h2, h3");
    }

    #[test]
    fn combination_id_selector() {
        let style: String = css!(div#hidden);
        assert_eq!(style, "p#hidden");
    }

    #[test]
    fn combination_class_selector() {
        let style: String = css!(p.hidden);
        assert_eq!(style, "p.hidden");
    }

    #[test]
    fn pseudo_class_selector() {
        let style: String = css!(div:hover);
        assert_eq!(style, "div:hover");
    }

    #[test]
    fn pseudo_element_selector() {
        let style: String = css!(div::after);
        assert_eq!(style, "div::after");
    }
}
