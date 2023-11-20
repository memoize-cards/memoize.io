use worker::*;

pub trait Handler {
    fn handle<T>(req: Request, tx: RouteContext<T>) -> Result<Response>;
}
