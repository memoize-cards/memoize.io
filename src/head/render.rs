//! # Render Module
//!
//! The `render` module contains functionality related to rendering.

use super::Head;
use crate::css::*;
use crate::html;
use std::fmt;

impl<'a> fmt::Display for Head<'a> {
    /// Implements the `fmt::Display` trait for the `Head` struct, allowing it to be formatted as HTML.
    ///
    /// # Example
    ///
    /// ```
    /// use your_crate_name::Head;
    ///
    /// let head = Head {
    ///     description: "Sample Description",
    ///     title: "Sample Title",
    /// };
    ///
    /// let formatted_head = format!("{}", head);
    /// // Assert the formatted head contains expected HTML structure.
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // HTML template using the `html!` macro.
        let template: String = html!(
            <head>
                <meta charset="UTF-8">
                <meta name="description" content={self.get_description()} />
                <meta name="theme-color" content="#fafafa" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>{self.get_title()}</title>
                <link rel="icon" href="//memoize.cards/favicon.ico" sizes="any" />
                <link rel="icon" href="//memoize.cards/memoize.svg" type="image/svg+xml" />
                <link rel="apple-touch-icon" href="//memoize.cards/memoize_180w.png" />
                <link rel="manifest" href="//memoize.cards/manifest.json" />
                <link rel="preconnect" href="//fonts.gstatic.com" crossorigin />
                <link rel="preconnect" href="//fonts.googleapis.com" crossorigin />
                <link rel="preconnect" href="//cdnjs.cloudflare.com" crossorigin />
                <link rel="stylesheet" href="//fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@48,300,0,0" />
                <link rel="stylesheet" href="//fonts.googleapis.com/css2?family=Roboto+Condensed:wght@400;500;700&family=Roboto:wght@400;500;700&display=swap" />
                {style()} // Include the style from the `style()` function.
            </head>
        );

        write!(f, "{}", template)
    }
}
