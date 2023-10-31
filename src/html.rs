//! Uma macro para geração eficiente de código HTML em Rust.
//!
//! A macro `html!` simplifica a criação de código HTML dentro do Rust de forma concisa.
//! Ela permite a construção de tags HTML e atributos de forma programática.
//!
//! # Exemplo
//!
//! ```
//! use html::html;
//!
//! let page = html!(
//!     <html>
//!         <head>
//!             <title>{"Minha Página HTML"}</title>
//!         </head>
//!         <body>
//!             <h1>{"Bem-vindo ao Rust HTML Macro!"}</h1>
//!             <p>{"Este é um exemplo de geração de HTML usando a macro `html!`."}</p>
//!         </body>
//!     </html>
//! );
//! println!("{}", page);
//! ```
//!
//! Este exemplo cria uma estrutura HTML simples usando a macro `html!` e a imprime.
//!
//! # Formato da Macro
//!
//! A macro `html!` segue um formato que permite a construção de elementos HTML de maneira concisa:
//!
//! - `<$tag $($look_ahead)*>`: Abre uma tag HTML com um nome de tag, seguido por atributos.
//! - `$attribute=$value $($look_ahead)*`: Define um atributo com uma chave e um valor.
//! - `data-$attribute- $($look_ahead)*`: Define um data atributo personalizado de dados.
//! - `/> $($look_ahead)*`: Fecha uma tag autossuficiente.
//! - `> $($look_ahead)*`: Fecha uma tag que pode conter conteúdo.
//! - `{$content} $($look_ahead)*`: Insere um valor diretamente no HTML.
//! - `</$tag $($look_ahead)*>`: Fecha uma tag HTML.
//! - `-$custom_element $($look_ahead)*`: Define um elemento HTML personalizado.
//! - `()` : Finaliza a construção do HTML.
//!
//! # Notas
//!
//! Esta macro é uma ferramenta poderosa para a geração eficiente de código HTML, mas deve ser usada com cuidado para evitar a injeção de código malicioso.
//!
//! Lembre-se de que o Rust é uma linguagem de programação com forte segurança de tipo, e a macro `html!` não faz verificações de segurança contra injeção de código, por isso é importante validar e sanitizar quaisquer dados inseridos no HTML gerado.
//!

#[macro_export]
macro_rules! html {
    (<$tag:ident $($look_ahead:tt)*) => {
        format!("<{}{}", stringify!($tag), html!($($look_ahead)*))
    };

    ($attribute:ident=$value:tt $($look_ahead:tt)*) => {
        format!(r#" {}="{}"{}"#, stringify!($attribute), $value, html!($($look_ahead)*))
    };

    (data-$attribute:ident=$value:tt $($look_ahead:tt)*) => {
        format!(r#" data-{}="{}"{}"#, stringify!($attribute), $value, html!($($look_ahead)*))
    };

    (/> $($look_ahead:tt)*) => {
        format!("/>{}", html!($($look_ahead)*))
    };

    (> $($look_ahead:tt)*) => {
        format!(">{}", html!($($look_ahead)*))
    };

    ({$content:expr} $($look_ahead:tt)*) => {
        format!("{}{}", $content, html!($($look_ahead)*))
    };

    (</$tag:ident $($look_ahead:tt)*) => {
        format!("</{}{}", stringify!($tag), html!($($look_ahead)*))
    };

    (-$custom_element:ident $($look_ahead:tt)*) => {
        format!("-{}{}", stringify!($custom_element), html!($($look_ahead)*))
    };

    () => { "" }
}

#[cfg(test)]
mod tests {
    #[test]
    fn open_tag() {
        let document: String = html!(<div);
        assert_eq!(document, "<div");
    }

    #[test]
    fn open_custom_tag() {
        let document: String = html!(<memoize-footer>);
        assert_eq!(document, "<memoize-footer>");
    }

    #[test]
    fn attributes() {
        let name: &str = "deMGoncalves";
        let document: String = html!(alt={name} src="./deMGoncalves.png");
        assert_eq!(document, r#" alt="deMGoncalves" src="./deMGoncalves.png""#);
    }

    #[test]
    fn data_attributes() {
        let document: String = html!(<img data-src="./deMGoncalves.png" />);
        assert_eq!(document, r#"<img data-src="./deMGoncalves.png"/>"#);
    }

    #[test]
    fn self_closing_tag() {
        let document: String = html!(/>);
        assert_eq!(document, "/>");
    }

    #[test]
    fn closing_tag() {
        let document: String = html!(>);
        assert_eq!(document, ">");
    }

    #[test]
    fn content() {
        let name: &str = "deMGoncalves";
        let document: String = html!({ format!("I'm {name}") });
        assert_eq!(document, "I'm deMGoncalves");
    }

    #[test]
    fn close_tag() {
        let document: String = html!(</div);
        assert_eq!(document, "</div");
    }

    #[test]
    fn close_custom_tag() {
        let document: String = html!(</memoize-footer>);
        assert_eq!(document, "</memoize-footer>");
    }
}
