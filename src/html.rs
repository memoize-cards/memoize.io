//! # HTML Macro
//!
//! The `html!` macro provides a convenient way to generate HTML content using a syntax similar to HTML.
//!
//! # Examples
//!
//! ```rust
//! // Open a div tag
//! let document: String = html!(<div);
//! // Output: "<div"
//!
//! // Create a custom HTML tag
//! let document: String = html!(<memoize-footer>);
//! // Output: "<memoize-footer>"
//!
//! // Add attributes to an HTML tag
//! let name: &str = "deMGoncalves";
//! let document: String = html!(alt={name} src="./deMGoncalves.png");
//! // Output: r#" alt="deMGoncalves" src="./deMGoncalves.png""#
//!
//! // Create a self-closing tag
//! let document: String = html!(/>);
//! // Output: "/>"
//!
//! // Create a closing tag
//! let document: String = html!(>);
//! // Output: ">"
//!
//! // Include content in an HTML tag
//! let name: &str = "deMGoncalves";
//! let document: String = html!({ format!("I'm {name}") });
//! // Output: "I'm deMGoncalves"
//!
//! // Close a tag
//! let document: String = html!(</div);
//! // Output: "</div"
//! ```

#[macro_export]
macro_rules! html {
    (<$tag:ident $($look_ahead:tt)*) => {
        format!("<{}{}", stringify!($tag), html!($($look_ahead)*))
    };

    ($attribute:ident=$value:tt $($look_ahead:tt)*) => {
        format!(r#" {}="{}"{}"#, stringify!($attribute), $value, html!($($look_ahead)*))
    };

    ($data:ident-$attribute:ident=$value:tt $($look_ahead:tt)*) => {
        format!(r#" {}-{}="{}"{}"#, stringify!($data), stringify!($attribute), $value, html!($($look_ahead)*))
    };

    (/> $($look_ahead:tt)*) => {
        format!("/>{}", html!($($look_ahead)*))
    };

    (> $($look_ahead:tt)*) => {
        format!(">{}", html!($($look_ahead)*))
    };

    ({$content:expr} $($look_ahead:tt)*) => {
        format!("{}{}", $content, html!($($look_ahead)*))
    };

    (</$tag:ident $($look_ahead:tt)*) => {
        format!("</{}{}", stringify!($tag), html!($($look_ahead)*))
    };

    ($attribute:ident $($look_ahead:tt)*) => {
        format!("{} {}", stringify!($attribute), html!($($look_ahead)*))
    };

    ($data_attribute:ident- $($look_ahead:tt)*) => {
        format!("{}-{}", stringify!($data_attribute), html!($($look_ahead)*))
    };

    (-$custom_element:ident $($look_ahead:tt)*) => {
        format!("-{}{}", stringify!($custom_element), html!($($look_ahead)*))
    };

    () => { "" }
}

#[cfg(test)]
mod tests {
    #[test]
    fn open_tag() {
        let document: String = html!(<div);
        assert_eq!(document, "<div");
    }

    #[test]
    fn open_custom_tag() {
        let document: String = html!(<memoize-footer>);
        assert_eq!(document, "<memoize-footer>");
    }

    #[test]
    fn attributes() {
        let name: &str = "deMGoncalves";
        let document: String = html!(alt={name} src="./deMGoncalves.png");
        assert_eq!(document, r#" alt="deMGoncalves" src="./deMGoncalves.png""#);
    }

    #[test]
    fn data_attributes() {
        let document: String = html!(<img data-src="./deMGoncalves.png" />);
        assert_eq!(document, r#"<img data-src="./deMGoncalves.png"/>"#);
    }

    #[test]
    fn self_closing_tag() {
        let document: String = html!(/>);
        assert_eq!(document, "/>");
    }

    #[test]
    fn closing_tag() {
        let document: String = html!(>);
        assert_eq!(document, ">");
    }

    #[test]
    fn content() {
        let name: &str = "deMGoncalves";
        let document: String = html!({ format!("I'm {name}") });
        assert_eq!(document, "I'm deMGoncalves");
    }

    #[test]
    fn close_tag() {
        let document: String = html!(</div);
        assert_eq!(document, "</div");
    }

    #[test]
    fn close_custom_tag() {
        let document: String = html!(</memoize-footer>);
        assert_eq!(document, "</memoize-footer>");
    }
}
