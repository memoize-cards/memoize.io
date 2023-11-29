use super::footer::language::Language;
mod language;
mod render;
pub mod styled;

pub struct Footer<'a> {
    benefits: &'a str,
    how_it_work: &'a str,
    try_it_out: &'a str,
    usage: &'a str,
    language: Language,
}

impl<'a> Footer<'a> {
    pub fn get_benefits(&self) -> &'a str {
        self.benefits
    }

    pub fn get_how_it_work(&self) -> &'a str {
        self.how_it_work
    }

    pub fn get_try_it_out(&self) -> &'a str {
        self.try_it_out
    }

    pub fn get_usage(&self) -> &'a str {
        self.usage
    }

    pub fn get_language(&self) -> &Language {
        &self.language
    }

    pub fn new() -> Footer<'a> {
        let language = Language::new();
        Footer {
            benefits: "Domine novas palavras, revise conceitos, prepare-se para exames e reforce seu aprendizado",
            how_it_work: "Organize informações facilmente usando cartões de estudo",
            try_it_out: "Simplicidade, foco no essencial e aprendizado em qualquer lugar",
            usage: "Revisite e reforce seu conhecimento com a repetição espaçada",
            language,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_benefits() {
        let footer = Footer::new();
        assert_eq!(
            footer.get_benefits(),
            "Domine novas palavras, revise conceitos, prepare-se para exames e reforce seu aprendizado"
        );
    }

    #[test]
    fn get_how_it_work() {
        let footer = Footer::new();
        assert_eq!(
            footer.get_how_it_work(),
            "Organize informações facilmente usando cartões de estudo"
        );
    }

    #[test]
    fn get_try_it_out() {
        let footer = Footer::new();
        assert_eq!(
            footer.get_try_it_out(),
            "Simplicidade, foco no essencial e aprendizado em qualquer lugar"
        );
    }

    #[test]
    fn get_usage() {
        let footer = Footer::new();
        assert_eq!(
            footer.get_usage(),
            "Revisite e reforce seu conhecimento com a repetição espaçada"
        );
    }
}
