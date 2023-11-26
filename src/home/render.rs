use super::Home;
use crate::head::Head;
use crate::html;
use std::fmt;

impl<'a> fmt::Display for Home<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let head = Head {
            description: "Aprenda, Memorize, Domine!",
            title: "Memoize",
        };

        let template: String = html!(
            <html lang="pt-BR" translate="no">
                {head}
                <body>
                    {self.get_header()}
                    {self.get_main()}
                    {self.get_footer()}
                </body>
            </html>
        );

        write!(f, "{}", template)
    }
}
