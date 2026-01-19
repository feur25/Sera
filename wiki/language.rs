use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProgrammingLanguage {
    Python,
    CSharp,
    Cpp,
}

impl ProgrammingLanguage {
    pub fn all() -> Vec<Self> {
        vec![Self::Python, Self::CSharp, Self::Cpp]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Python => "Python",
            Self::CSharp => "C#",
            Self::Cpp => "C++",
        }
    }

    pub fn file_extension(&self) -> &'static str {
        match self {
            Self::Python => "py",
            Self::CSharp => "cs",
            Self::Cpp => "cpp",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeExample {
    pub python: String,
    pub csharp: String,
    pub cpp: String,
}

impl CodeExample {
    pub fn new(python: &str, csharp: &str, cpp: &str) -> Self {
        Self {
            python: python.to_string(),
            csharp: csharp.to_string(),
            cpp: cpp.to_string(),
        }
    }

    pub fn get(&self, lang: &ProgrammingLanguage) -> &str {
        match lang {
            ProgrammingLanguage::Python => &self.python,
            ProgrammingLanguage::CSharp => &self.csharp,
            ProgrammingLanguage::Cpp => &self.cpp,
        }
    }
}
