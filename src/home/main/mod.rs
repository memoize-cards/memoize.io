use super::main::copy_write::CopyWrite;
use super::main::github::Github;
use super::main::illustration::Illustration;
use super::main::linkedin::LinkedIn;
use super::main::privacy_policy::PrivacyPolicy;
use super::main::terms_of_use::TermsOfUse;
mod copy_write;
mod github;
mod illustration;
mod linkedin;
mod privacy_policy;
mod render;
pub mod styled;
mod terms_of_use;

pub struct Main<'a> {
    description: &'a str,
    title: &'a str,
    copy_write: CopyWrite<'a>,
    github: Github<'a>,
    illustration: Illustration<'a>,
    linkedin: LinkedIn<'a>,
    privacy_policy: PrivacyPolicy<'a>,
    terms_of_use: TermsOfUse<'a>,
}

impl<'a> Main<'a> {
    pub fn get_copy_write(&self) -> &CopyWrite {
        &self.copy_write
    }

    pub fn get_description(&self) -> &'a str {
        self.description
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

    pub fn get_title(&self) -> &'a str {
        self.title
    }

    pub fn get_privacy_policy(&self) -> &PrivacyPolicy {
        &self.privacy_policy
    }

    pub fn get_terms_of_use(&self) -> &TermsOfUse {
        &self.terms_of_use
    }

    pub fn new() -> Main<'a> {
        let copy_write = CopyWrite::new();
        let github = Github::new();
        let illustration = Illustration::new();
        let linkedin = LinkedIn::new();
        let privacy_policy = PrivacyPolicy::new();
        let terms_of_use = TermsOfUse::new();
        Main {
            description: "Um aplicativo avançado de flashcard projetado para otimizar o processo de aprendizado e memorização",
            title: "Aprenda, Memorize, Domine!",
            copy_write,
            github,
            illustration,
            linkedin,
            privacy_policy,
            terms_of_use,
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
