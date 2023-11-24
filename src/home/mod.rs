//! # Home Module
//!
//! The `home` module contains functionality related to the home route.

use crate::head::Head;
mod handle;
mod render;
pub mod styled;

/// Represents the home route.
///
/// # Examples
///
/// ```
/// use crate::Home;
/// use crate::head::Head;
///
/// let head = Head {
///     title: "Memoize",
///     description: "Aprenda, Memorize, Domine!",
/// };
///
/// let home = Home { head: &head };
/// assert_eq!(home.get_head(), &head);
/// ```
pub struct Home<'a> {
    /// The head section of the home route.
    head: &'a Head<'a>,
}

impl<'a> Home<'a> {
    /// Gets the head section of the home route.
    ///
    /// # Returns
    ///
    /// The head section of the home route.
    pub fn get_head(&self) -> &'a Head<'a> {
        self.head
    }
}
