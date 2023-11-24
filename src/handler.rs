//! # Handler Trait
//!
//! The `Handler` trait defines the interface for request handling.

use worker::*;

/// A trait for handling HTTP requests.
pub trait Handler {
    /// Handles an HTTP request.
    ///
    /// # Arguments
    ///
    /// * `req` - The HTTP request.
    /// * `ctx` - The context for routing.
    ///
    /// # Returns
    ///
    /// A `Result` containing the HTTP response.
    fn handle<T>(req: Request, ctx: RouteContext<T>) -> Result<Response>;
}
