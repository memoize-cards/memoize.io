//! # Handler Module
//!
//! The `handler` module contains functionality related to request handling.

use super::Home;
use crate::handler::Handler;
use crate::head::Head;
use worker::*;

impl<'a> Handler for Home<'a> {
    /// Implements the `Handler` trait for the `Home` struct, defining how requests to the home route should be handled.
    ///
    /// # Arguments
    ///
    /// - `_req`: The incoming request.
    /// - `_ctx`: The route context.
    ///
    /// # Returns
    ///
    /// A `Result` containing the generated `Response`.
    ///
    /// # Example
    ///
    /// ```
    /// use your_crate_name::{Home, Handler};
    /// use worker::{Request, RouteContext, Result, Response};
    ///
    /// // Simulate incoming request and route context.
    /// let req: Request = /* create request */;
    /// let ctx: RouteContext<_> = /* create route context */;
    ///
    /// // Handle the request using the `Home` struct.
    /// let result = Home::handle(req, ctx);
    /// // Process the result, e.g., send the response to the client.
    /// ```
    fn handle<T>(_req: Request, _ctx: RouteContext<T>) -> Result<Response> {
        // Create a `Head` instance with title and description.
        let head: Head = Head {
            title: "Memoize",
            description: "Aprenda, Memorize, Domine!",
        };

        // Create a `Home` instance with the generated `Head`.
        let home: Home = Home { head: &head };

        // Format the `Home` instance as HTML and create a `Response`.
        Response::from_html(format!("{home}"))
    }
}
