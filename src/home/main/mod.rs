use super::main::github::Github;
use super::main::illustration::Illustration;
use super::main::linkedin::LinkedIn;
mod github;
mod illustration;
mod linkedin;
mod render;
pub mod styled;

pub struct Main<'a> {
    description: &'a str,
    title: &'a str,
    github: Github<'a>,
    illustration: Illustration<'a>,
    linkedin: LinkedIn<'a>,
}

impl<'a> Main<'a> {
    pub fn get_description(&self) -> &'a str {
        self.description
    }

    pub fn get_title(&self) -> &'a str {
        self.title
    }

    pub fn get_github(&self) -> &Github<'a> {
        &self.github
    }

    pub fn get_illustration(&self) -> &Illustration<'a> {
        &self.illustration
    }

    pub fn get_linkedin(&self) -> &LinkedIn<'a> {
        &self.linkedin
    }

    pub fn new() -> Main<'a> {
        let github = Github::new();
        let illustration = Illustration::new();
        let linkedin = LinkedIn::new();
        Main {
            description: "Um aplicativo avançado de flashcard projetado para otimizar o processo de aprendizado e memorização",
            title: "Aprenda, Memorize, Domine!",
            github,
            illustration,
            linkedin
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
