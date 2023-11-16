use std::fmt;
use worker::*;

// Defina um componente reutilizável
struct Header<'a> {
    title: &'a str,
}

// Implemente a trait Display para o componente
impl<'a> fmt::Display for Header<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<head><title>{}</title></head>", self.title)
    }
}

// Struct principal que incorpora o componente Header
struct MyPage<'a> {
    header: Header<'a>,
    content: &'a str,
}

// Implemente a trait Display para a struct principal
impl<'a> fmt::Display for MyPage<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<html>{}<body><h1>{}</h1><p>{}</p></body></html>",
            self.header, self.header.title, self.content
        )
    }
}

// Implemente métodos para manipular a struct principal
impl<'a> MyPage<'a> {
    // Construtor
    fn new(title: &'a str, content: &'a str) -> MyPage<'a> {
        let header = Header { title };
        MyPage { header, content }
    }

    // Método para renderizar o template
    fn render(&self) -> String {
        format!("{}", self)
    }
}

#[event(fetch)]
async fn main(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    // Crie uma instância da struct principal
    let my_page = MyPage::new("Minha Página", "Conteúdo da página.");

    // Chame o método render para obter o template
    let template = my_page.render();

    Response::from_html(format!("{}", template))
}
