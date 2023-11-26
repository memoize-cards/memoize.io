static mut CONTENT: String = String::new();

pub fn get_content() -> String {
    unsafe { CONTENT.clone() }
}

pub fn push(content: &str) {
    let content: String = content
        .replace('\n', " ")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    unsafe {
        CONTENT.push_str(&content);
    }
}

#[cfg(test)]
mod tests {
    use crate::css;

    #[test]
    fn push_and_get_content() {
        css::push(
            r#"
            * {
                all: unset;
            }
        "#,
        );
        assert_eq!(css::get_content(), "* { all: unset; }");
    }
}
