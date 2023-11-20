#![recursion_limit = "256"]

use crate::handler::Handler;
use worker::*;
mod handler;
mod home;
mod reset;
mod tokens;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router.get("/", home::Home::handle).run(req, env).await
}

#[event(start)]
fn start() {
    reset::init();
    tokens::init();
    home::style::init();
}
