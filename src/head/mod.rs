//! # Struct Head para Geração de Documentos HTML
//!
//! O módulo `template` contém a definição da estrutura `Head` em Rust, que é utilizada para representar a seção
//! `<head>` de um documento HTML, incluindo descrição e título.
//!
//! ## Struct `Head`
//!
//! A struct `Head` possui os seguintes campos:
//!
//! - `description`: Uma referência a uma string (`&str`) que armazena a descrição do documento HTML.
//! - `title`: Uma referência a uma string (`&str`) que armazena o título do documento HTML.
//!
//! ## Métodos
//!
//! A implementação da struct `Head` inclui os seguintes métodos:
//!
//! - `get_description(&self) -> &'a str`: Retorna a descrição do documento.
//! - `get_title(&self) -> &'a str`: Retorna o título do documento.
//!
//! ## Exemplo de Uso
//!
//! ```rust
//! use crate::Head;
//!
//! let head = Head {
//!     description: "Descrição do Documento",
//!     title: "Título do Documento",
//! };
//!
//! println!("Descrição: {}", head.get_description());
//! println!("Título: {}", head.get_title());
//! ```
mod render;

pub struct Head<'a> {
    pub description: &'a str,
    pub title: &'a str,
}

impl<'a> Head<'a> {
    pub fn get_description(&self) -> &'a str {
        self.description
    }

    pub fn get_title(&self) -> &'a str {
        self.title
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_description() {
        let head = Head {
            description: "Teste de Descrição",
            title: "Teste de Título",
        };
        assert_eq!(head.get_description(), "Teste de Descrição");
    }

    #[test]
    fn test_get_title() {
        let head = Head {
            description: "Teste de Descrição",
            title: "Teste de Título",
        };
        assert_eq!(head.get_title(), "Teste de Título");
    }
}
