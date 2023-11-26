use super::header::Header;
use super::Home;
use crate::handler::Handler;
use crate::head::Head;
use worker::*;

impl<'a> Handler for Home<'a> {
    fn handle<T>(_req: Request, _ctx: RouteContext<T>) -> Result<Response> {
        let head = Head {
            description: "Aprenda, Memorize, Domine!",
            title: "Memoize",
        };
        let header = Header::new();
        let home = Home::new(&head, &header);
        Response::from_html(format!("{home}"))
    }
}
