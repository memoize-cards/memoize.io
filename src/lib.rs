#![recursion_limit = "2046"]

use crate::handler::Handler;
use worker::*;
mod css;
mod handler;
mod head;
mod home;
mod html;
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
    home::styled::init();
}
