use super::Home;
use crate::handler::Handler;
use crate::head::Head;
use worker::*;

impl<'a> Handler for Home<'a> {
    fn handle<T>(_req: Request, _ctx: RouteContext<T>) -> Result<Response> {
        let head: Head = Head {
            title: "Memoize",
            description: "Aprenda, Memorize, Domine!",
        };

        let home: Home = Home { head: &head };

        Response::from_html(format!("{home}"))
    }
}
