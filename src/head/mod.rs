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
    fn get_description() {
        let head = Head {
            description: "Teste de Descrição",
            title: "Teste de Título",
        };
        assert_eq!(head.get_description(), "Teste de Descrição");
    }

    #[test]
    fn get_title() {
        let head = Head {
            description: "Teste de Descrição",
            title: "Teste de Título",
        };
        assert_eq!(head.get_title(), "Teste de Título");
    }
}
