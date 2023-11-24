//! # Implementação de fmt::Display para a Struct Head
//!
//! Este módulo contém a implementação do trait `fmt::Display` para a struct `Head`, permitindo que ela seja
//! formatada como uma string para ser usada na geração de documentos HTML.
//!
//! ## Implementação `fmt::Display` para `Head`
//!
//! A implementação adiciona os elementos essenciais à seção `<head>` de um documento HTML, incluindo metadados,
//! título, ícones e links para folhas de estilo externas.
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
//! println!("{}", head);
//! ```
//!
//! O exemplo acima demonstra como a struct `Head` pode ser formatada como uma string HTML usando a implementação
//! de `fmt::Display`.
//!
//! ## Estilo CSS
//!
//! O estilo CSS utilizado é importado da função `style()` e adicionado à seção `<head>` do documento gerado.
//!
//! ## Observação
//!
//! Certifique-se de que as URLs utilizadas nos links estejam acessíveis e atualizadas conforme necessário.
use super::Head;
use crate::css::*;
use crate::html;
use std::fmt;

impl<'a> fmt::Display for Head<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let template: String = html!(
            <head>
                <meta charset="UTF-8">
                <meta name="description" content={self.get_description()} />
                <meta name="theme-color" content="#fafafa" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>{self.get_title()}</title>
                <link rel="icon" href="//memoize.cards/favicon.ico" sizes="any" />
                <link rel="icon" href="//memoize.cards/memoize.svg" type="image/svg+xml" />
                <link rel="apple-touch-icon" href="//memoize.cards/memoize_180w.png" />
                <link rel="manifest" href="//memoize.cards/manifest.json" />
                <link rel="preconnect" href="//fonts.gstatic.com" crossorigin />
                <link rel="preconnect" href="//fonts.googleapis.com" crossorigin />
                <link rel="preconnect" href="//cdnjs.cloudflare.com" crossorigin />
                <link rel="stylesheet" href="//fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@48,300,0,0" />
                <link rel="stylesheet" href="//fonts.googleapis.com/css2?family=Roboto+Condensed:wght@400;500;700&family=Roboto:wght@400;500;700&display=swap" />
                {style()}
            </head>
        );

        write!(f, "{}", template)
    }
}
