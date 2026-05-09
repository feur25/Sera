use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProgrammingLanguage {
    Python,
    CSharp,
    Cpp,
    Rust,
}

impl ProgrammingLanguage {
    pub fn all() -> Vec<Self> {
        vec![Self::Python, Self::CSharp, Self::Cpp, Self::Rust]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Python => "Python",
            Self::CSharp => "C#",
            Self::Cpp => "C++",
            Self::Rust => "Rust",
        }
    }

    pub fn file_extension(&self) -> &'static str {
        match self {
            Self::Python => "py",
            Self::CSharp => "cs",
            Self::Cpp => "cpp",
            Self::Rust => "rs",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeExample {
    pub python: String,
    pub csharp: String,
    pub cpp: String,
    pub rust: String,
}

impl CodeExample {
    pub fn new(python: &str, csharp: &str, cpp: &str, rust: &str) -> Self {
        Self {
            python: python.to_string(),
            csharp: csharp.to_string(),
            cpp: cpp.to_string(),
            rust: rust.to_string(),
        }
    }

    pub fn get(&self, lang: &ProgrammingLanguage) -> &str {
        match lang {
            ProgrammingLanguage::Python => &self.python,
            ProgrammingLanguage::CSharp => &self.csharp,
            ProgrammingLanguage::Cpp => &self.cpp,
            ProgrammingLanguage::Rust => &self.rust,
        }
    }
}


