//! Uma macro para geração eficiente de código HTML em Rust.
//!
//! A macro `html!` simplifica a criação de código HTML dentro do Rust de forma concisa.
//! Ela permite a construção de tags HTML e atributos de forma programática.

#[macro_export]
macro_rules! html {
    (<$tag:ident $($look_ahead:tt)*) => {
        format!("<{}{}", stringify!($tag), html!($($look_ahead)*))
    };

    (<$custom:ident-$element:ident $($look_ahead:tt)*) => {
        format!("<{}-{}{}", stringify!($custom), stringify!($element), html!($($look_ahead)*))
    };

    ($key:ident=$value:tt $($look_ahead:tt)*) => {
        format!(" {}='{}'{}", stringify!($key), $value, html!($($look_ahead)*))
    };

    (/> $($look_ahead:tt)*) => {
        format!(" />{}", html!($($look_ahead)*))
    };

    (> $($look_ahead:tt)*) => {
        format!(">{}", html!($($look_ahead)*))
    };

    ({$value:expr} $($look_ahead:tt)*) => {
        format!("{}{}", $value, html!($($look_ahead)*))
    };

    ($content:literal $($look_ahead:tt)*) => {
        format!("{}{}", $content, html!($($look_ahead)*))
    };

    (</$tag:ident $($look_ahead:tt)*) => {
        format!("</{}{}", stringify!($tag), html!($($look_ahead)*))
    };

    (</$custom:ident-$element:ident $($look_ahead:tt)*) => {
        format!("</{}-{}{}", stringify!($custom), stringify!($element), html!($($look_ahead)*))
    };

    () => { "" }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_open_tag() {
        let document: String = html!(
            <div>
        );
        assert_eq!(document, "<div>");
    }

    #[test]
    fn test_close_tag() {
        let document: String = html!(
            </div>
        );
        assert_eq!(document, "</div>");
    }

    #[test]
    fn test_self_close_tag() {
        let document: String = html!(
            <img />
        );
        assert_eq!(document, "<img />");
    }

    #[test]
    fn test_attributes() {
        let name: &str = "deMGoncalves";
        let document: String = html!(
            <img alt={name} src="./deMGoncalves.png" />
        );
        assert_eq!(
            document,
            "<img alt='deMGoncalves' src='./deMGoncalves.png' />"
        );
    }

    #[test]
    fn test_content() {
        let name: &str = "deMGoncalves";
        let document: String = html!({ format!("I'm {}", name) });
        assert_eq!(document, "I'm deMGoncalves");
    }

    #[test]
    fn test_multiple_elements() {
        let name: &str = "deMGoncalves";
        let document: String = html!(
            <div class="avatar">
                <img alt={name} src="./deMGoncalves.png" />
                <strong>{format!("I'm {}", name)}</strong>
            </div>
        );
        assert_eq!(
            document,
            "<div class='avatar'><img alt='deMGoncalves' src='./deMGoncalves.png' /><strong>I'm deMGoncalves</strong></div>"
        );
    }

    #[test]
    #[ignore = "Preciso pensa em um test de web component"]
    fn test_custom_element_tag() {}
}
