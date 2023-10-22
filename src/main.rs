mod html;

fn main() {
    let name: &str = "deMGoncalves";
    let document = html!(
        <div class="avatar">
            <img alt={name} src="./deMGoncalves.png" />
            <strong>"I'm deMGoncalves"</strong>
        </div>
    );
    println!("{}", document);
}
