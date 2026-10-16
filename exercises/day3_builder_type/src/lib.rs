#[derive(Debug, PartialEq, Eq)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
}

pub struct PackageBuilder {
    name: String,
    version: Option<String>,
    authors: Vec<String>,
}

impl PackageBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        PackageBuilder {
            name: name.into(),
            version: None,
            authors: Vec::new(),
        }
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.authors.push(author.into());
        self
    }

    pub fn build(self) -> Package {
        Package {
            name: self.name,
            version: self.version.unwrap_or_else(|| String::from("0.1.0")),
            authors: self.authors,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let pkg = PackageBuilder::new("serde")
            .version("1.0.0")
            .author("dtolnay")
            .build();

        assert_eq!(pkg.name, "serde");
        assert_eq!(pkg.version, "1.0.0");
        assert_eq!(pkg.authors, vec!["dtolnay"]);
    }
}
