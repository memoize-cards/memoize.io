use super::Home;
use css;
use html::*;
use std::fmt;

impl<'a> fmt::Display for Home<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let html_content: String = html!(
            <html lang="en-US" translate="no">
                <head>
                    <meta charset="UTF-8">
                    <meta name="description" content={self.description()} />
                    <meta name="theme-color" content="#fafafa" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <title>{self.title()}</title>
                    <link rel="icon" href="//memoize.cards/favicon.ico" sizes="any" />
                    <link rel="icon" href="//memoize.cards/memoize.svg" type="image/svg+xml" />
                    <link rel="apple-touch-icon" href="//memoize.cards/memoize_180w.png" />
                    <link rel="manifest" href="//memoize.cards/manifest.json" />
                    <link rel="preconnect" href="//fonts.gstatic.com" crossorigin />
                    <link rel="preconnect" href="//fonts.googleapis.com" crossorigin />
                    <link rel="preconnect" href="//cdnjs.cloudflare.com" crossorigin />
                    <link rel="stylesheet" href="//fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@48,300,0,0" />
                    <link rel="stylesheet" href="//fonts.googleapis.com/css2?family=Roboto+Condensed:wght@400;500;700&family=Roboto:wght@400;500;700&display=swap" />
                    {css::style()}
                </head>
                <body>
                    <h1>{self.title()}</h1>
                </body>
            </html>
        );
        write!(f, "{}", html_content)
    }
}
