use super::Home;
use crate::handler::Handler;
use worker::*;

impl<'a> Handler for Home<'a> {
    fn handle<T>(_req: Request, _ctx: RouteContext<T>) -> Result<Response> {
        let home = Home::new();
        let template = format!("<!doctype html>{home}");
        Response::from_html(template)
    }
}
