use super::Head;
use crate::css;
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
                <link rel="icon" href="//assets.memoize.cards/favicon.ico" sizes="any" />
                <link rel="icon" href="//assets.memoize.cards/memoize.svg" type="image/svg+xml" />
                <link rel="apple-touch-icon" href="//assets.memoize.cards/memoize_180w.png" />
                <link rel="manifest" href="//assets.memoize.cards/manifest.json" />
                <link rel="preconnect" href="//assets.memoize.cards" crossorigin />
                <style>{css::get_content()}</style>
            </head>
        );

        write!(f, "{}", template)
    }
}
