//! This module provides functions for manipulating CSS styles in the application.

use crate::css::*;

/// Initializes the default CSS style for the application.
///
/// This function applies common styles to all elements, such as `box-sizing`,
/// `margin`, and `padding`, and sets the background color of the body.
///
/// # Example
///
/// ```rust
/// use crate::css::init;
///
/// // Initialize the CSS style
/// init();
/// ```
pub fn init() {
    css(r#"
        *,
        *::after,
        *::before {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }

        body {
            background-color: var(--color-master-lightest);
        }
    "#);
}
