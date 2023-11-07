mod css;
mod html;

fn main() {
    let name: &str = "deMGoncalves";
    let document: String = html!(
        <html>
            <head>
                {css::style()}
            </head>
            <body>
                <div class="avatar">
                    <img alt={name} src="./deMGoncalves.png" />
                    <strong>{format!("I'm {name}")}</strong>
                </div>
            </body>
        </html>
    );
    println!("{}", document);
}
