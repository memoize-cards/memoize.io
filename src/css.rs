static mut CONTENT: String = String::new();

pub fn css(content: &str) -> String {
    let content: String = content
        .replace("\n", " ")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    unsafe {
        CONTENT.push_str(&content);
    }

    content
}

pub fn style_tag() -> String {
    unsafe { format!("<style>{CONTENT}</style>") }
}

#[cfg(test)]
mod tests {
    use crate::css::*;

    #[test]
    fn minify_content() {
        let style: String = css(r#"
            * {
                all: unset;
            }
        "#);
        assert_eq!(style, "* { all: unset; }");
    }

    #[test]
    fn global_content() {
        let tag: String = style_tag();
        assert_eq!(tag, "<style>* { all: unset; }</style>")
    }
}
