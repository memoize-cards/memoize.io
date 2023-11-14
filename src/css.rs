//! Biblioteca para minificar e gerar estilos CSS de forma eficiente em Rust.
//!
//! A biblioteca `css` oferece funcionalidades para minificar e gerar estilos CSS de forma simplificada em Rust.
//! Você pode usar as funções fornecidas para processar conteúdo CSS, remover quebras de linha, tabulações e espaços em branco duplicados,
//! e gerar tags de estilo HTML que podem ser incorporadas em páginas da web.
//!
//! # Exemplo
//!
//! ```
//! use css::css;
//!
//! let style = css(r#"
//!     * {
//!         all: unset;
//!     }
//! "#);
//!
//! let style_tag = style();
//!
//! println!("{}", style);
//! println!("{}", style_tag);
//! ```
//!
//! Este exemplo demonstra o uso das funções da biblioteca `css` para processar conteúdo CSS e gerar tags de estilo HTML.
//!
//! # Funções
//!
//! - `css(content: &str) -> String`: Recebe uma string de conteúdo CSS e a transforma em uma versão minificada,
//! removendo quebras de linha, tabulações e espaços em branco duplicados. Retorna a versão minificada do CSS.
//!
//! - `style() -> String`: Gera uma tag de estilo HTML que contém o CSS processado pelas chamadas à função `css()`.
//! Retorna a tag de estilo pronta para ser incorporada em uma página da web.
//!
//! # Notas
//!
//! A biblioteca `css` é uma ferramenta útil para otimizar estilos CSS e gerar tags de estilo para páginas web.
//! Lembre-se de que o conteúdo CSS deve ser cuidadosamente validado e sanitizado para evitar injeção de código malicioso
//! quando incorporado em páginas web.

/// Conteúdo CSS acumulado.
///
/// A variável global `CONTENT` é usada internamente para acumular o conteúdo CSS gerado
/// pela função `css()`. Ela permite a concatenação de estilos à medida que mais conteúdo é processado.
/// No entanto, o acesso e a manipulação direta desta variável devem ser feitos com cautela, e não é recomendado
/// a menos que seja necessário para cenários avançados.
///
/// # Exemplo
///
/// ```rust
/// use css::css;
///
/// let style1 = css(r#"
///     * {
///         all: unset;
///     }
/// "#);
///
/// let style2 = css(r#"
///     body {
///         background-color: #f0f0f0;
///     }
/// "#);
///
/// println!("Estilos acumulados: {}", css::CONTENT);
/// ```
static mut CONTENT: String = String::new();

/// Minifica o conteúdo CSS, removendo quebras de linha, tabulações e espaços em branco duplicados.
///
/// # Argumentos
///
/// - `content`: Uma string contendo o conteúdo CSS a ser minificado.
///
/// # Exemplo
///
/// ```
/// use css::css;
///
/// let style = css(r#"
///     * {
///         all: unset;
///     }
/// "#);
///
/// assert_eq!(style, "* { all: unset; }");
/// ```
///
/// Este exemplo demonstra o uso da função `css()` para minificar o conteúdo CSS.
///
/// # Retorno
///
/// Uma string contendo o conteúdo CSS minificado.
pub fn css(content: &str) -> String {
    let content: String = content
        .replace("\n", " ")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    unsafe {
        CONTENT.push_str(&content);
    }

    content
}

/// Gera uma tag de estilo HTML que contém o CSS processado pelas chamadas à função `css()`.
///
/// # Exemplo
///
/// ```
/// use css::style;
///
/// let style_tag = style();
///
/// assert_eq!(style_tag, "<style>* { all: unset; }</style>");
/// ```
///
/// Este exemplo demonstra o uso da função `style()` para gerar uma tag de estilo HTML.
///
/// # Retorno
///
/// Uma string contendo a tag de estilo HTML pronta para ser incorporada em uma página da web.
pub fn style() -> String {
    unsafe { format!("<style>{CONTENT}</style>") }
}

#[cfg(test)]
mod tests {
    use crate::css::*;

    #[test]
    fn text_content() {
        let style: String = css(r#"
            * {
                all: unset;
            }
        "#);
        assert_eq!(style, "* { all: unset; }");
    }

    #[test]
    fn html_style_element() {
        css(r#"
            * {
                all: unset;
            }
        "#);
        let tag: String = style();
        assert_eq!(tag, "<style>* { all: unset; }</style>")
    }
}
