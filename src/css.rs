//! # CSS Module
//!
//! The `css` module provides functionality for handling CSS content.

/// Stores the CSS content as a mutable static string.
static mut CONTENT: String = String::new();

/// Processes and stores the provided CSS content.
///
/// # Arguments
///
/// * `content` - A string containing CSS content.
///
/// # Returns
///
/// The processed CSS content.
pub fn css(content: &str) -> String {
    // Remove newline characters and collapse consecutive whitespace into a single space
    let content: String = content
        .replace('\n', " ")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    // Use unsafe to modify the static CONTENT string
    unsafe {
        CONTENT.push_str(&content);
    }

    content
}

/// Generates an HTML style element with the stored CSS content.
///
/// # Returns
///
/// The HTML style element string.
pub fn style() -> String {
    // Use unsafe to access the static CONTENT string
    unsafe { format!("<style>{}</style>", CONTENT) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_processing() {
        let style: String = css(r#"
            * {
                all: unset;
            }
        "#);
        assert_eq!(style, "* { all: unset; }");
    }

    #[test]
    fn test_html_style_element_generation() {
        css(r#"
                * {
                all: unset;
            }
        "#);
        let tag: String = style();
        assert_eq!(tag, "<style>* { all: unset; }</style>")
    }
}
