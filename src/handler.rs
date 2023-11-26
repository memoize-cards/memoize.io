use worker::*;

pub trait Handler {
    fn handle<T>(req: Request, ctx: RouteContext<T>) -> Result<Response>;
}
