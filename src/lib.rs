use worker::*;

mod css;
mod html;

#[event(fetch)]
async fn main(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    css::css(
        r#"
        * {
            margin: 0;
            padding: 0;
        }
    "#,
    );

    let body: String = html!(
        <html>
            <head>
                <meta charset="UTF-8" />
                <title>{"Memoize"}</title>
                {css::style()}
            </head>
            <body>
                <div class="avatar">
                    <img alt="Memoize" src="./memoize.svg" />
                </div>
            </body>
        </html>
    );

    Response::from_html(body)
}
