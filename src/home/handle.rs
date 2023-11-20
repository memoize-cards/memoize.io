use super::Home;
use crate::handler::Handler;
use worker::*;

impl<'a> Handler for Home<'a> {
    fn handle<T>(_req: Request, _ctx: RouteContext<T>) -> Result<Response> {
        let home: Home = Home {
            title: "Memoize",
            description: "Aprenda, Memorize, Domine!",
        };

        Response::from_html(format!("{home}"))
    }
}
