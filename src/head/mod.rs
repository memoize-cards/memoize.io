//! # Render Module
//!
//! The `render` module contains functionality related to rendering.
mod render;

/// Represents the head section of a document.
///
/// # Examples
///
/// ```
/// use crate::Head;
///
/// let head = Head {
///     description: "Sample Description",
///     title: "Sample Title",
/// };
///
/// assert_eq!(head.get_description(), "Sample Description");
/// assert_eq!(head.get_title(), "Sample Title");
/// ```
pub struct Head<'a> {
    /// The description of the document head.
    pub description: &'a str,
    /// The title of the document head.
    pub title: &'a str,
}

impl<'a> Head<'a> {
    /// Gets the description of the document head.
    ///
    /// # Returns
    ///
    /// The description of the document head.
    pub fn get_description(&self) -> &'a str {
        self.description
    }

    /// Gets the title of the document head.
    ///
    /// # Returns
    ///
    /// The title of the document head.
    pub fn get_title(&self) -> &'a str {
        self.title
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_description() {
        let head = Head {
            description: "Teste de Descrição",
            title: "Teste de Título",
        };
        assert_eq!(head.get_description(), "Teste de Descrição");
    }

    #[test]
    fn test_get_title() {
        let head = Head {
            description: "Teste de Descrição",
            title: "Teste de Título",
        };
        assert_eq!(head.get_title(), "Teste de Título");
    }
}
