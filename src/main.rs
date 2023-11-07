mod css;
mod html;

fn main() {
    let name: &str = "deMGoncalves";
    css::css(
        r#"
        .avatar {
            all: unset;
        }
    "#,
    );
    let document: String = html!(
        {css::style_tag()}
        <div class="avatar">
            <img alt={name} src="./deMGoncalves.png" />
            <strong>{format!("I'm {name}")}</strong>
        </div>
    );
    println!("{}", document);
}
