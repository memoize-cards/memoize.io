mod css;
mod html;

fn main() {
    let name: &str = "deMGoncalves";
    let document: String = html!(
        <div class="avatar">
            <img alt={name} src="./deMGoncalves.png" />
            <strong>{format!("I'm {name}")}</strong>
        </div>
    );
    println!("{}", document);
}
