use super::main::illustration::Illustration;
mod illustration;
mod render;
pub mod styled;

pub struct Main<'a> {
    description: &'a str,
    title: &'a str,
    illustration: Illustration<'a>,
}

impl<'a> Main<'a> {
    pub fn get_description(&self) -> &'a str {
        self.description
    }

    pub fn get_title(&self) -> &'a str {
        self.title
    }

    pub fn get_illustration(&self) -> &Illustration<'a> {
        &self.illustration
    }

    pub fn new() -> Main<'a> {
        let illustration = Illustration::new();
        Main {
            description: "Um aplicativo avançado de flashcard projetado para otimizar o processo de aprendizado e memorização",
            title: "Aprenda, Memorize, Domine!",
            illustration,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_description() {
        let main = Main::new();
        assert_eq!(main.get_description(), "Um aplicativo avançado de flashcard projetado para otimizar o processo de aprendizado e memorização");
    }

    #[test]
    fn get_title() {
        let main = Main::new();
        assert_eq!(main.get_title(), "Aprenda, Memorize, Domine!");
    }
}
