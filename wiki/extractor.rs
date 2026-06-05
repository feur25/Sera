use super::metadata::{MethodDoc, ModuleDoc, ParamDoc};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct WikiExtractor {
    registry: HashMap<String, Vec<MethodDoc>>,
}

impl WikiExtractor {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
        }
    }

    pub fn register_method(&mut self, module: String, method: MethodDoc) {
        self.registry
            .entry(module)
            .or_insert_with(Vec::new)
            .push(method);
    }

    pub fn register_methods(&mut self, module: String, methods: Vec<MethodDoc>) {
        self.registry.insert(module, methods);
    }

    pub fn build_modules(&self) -> Vec<ModuleDoc> {
        self.registry
            .iter()
            .map(|(name, methods)| ModuleDoc {
                name: name.clone(),
                description: format!("{} module", name),
                methods: methods.clone(),
            })
            .collect()
    }

    pub fn export_json<P: AsRef<Path>>(
        &self,
        path: P,
        export: &crate::wiki::WikiExport,
    ) -> std::io::Result<()> {
        let json = export
            .to_json()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(path, json)
    }

    pub fn export_markdown<P: AsRef<Path>>(
        &self,
        path: P,
        export: &crate::wiki::WikiExport,
    ) -> std::io::Result<()> {
        let lang = crate::wiki::ProgrammingLanguage::Python;
        let md = export.to_markdown(&lang);
        fs::write(path, md)
    }

    pub fn export_html<P: AsRef<Path>>(
        &self,
        path: P,
        export: &crate::wiki::WikiExport,
    ) -> std::io::Result<()> {
        let lang = crate::wiki::ProgrammingLanguage::Python;
        let html = export.to_html(&lang);
        fs::write(path, html)
    }

    pub fn count_methods(&self) -> usize {
        self.registry.values().map(|m| m.len()).sum()
    }
}

impl Default for WikiExtractor {
    fn default() -> Self {
        Self::new()
    }
}

pub fn create_method_doc(
    name: &str,
    module: &str,
    python_sig: &str,
    description: &str,
) -> MethodDoc {
    MethodDoc {
        name: name.to_string(),
        module: module.to_string(),
        description: description.to_string(),
        parameters: Vec::new(),
        returns: None,
        examples: Vec::new(),
        since_version: None,
        deprecated: false,
        python_signature: python_sig.to_string(),
        csharp_signature: String::new(),
        cpp_signature: String::new(),
        rust_signature: String::new(),
    }
}

pub fn create_param_doc(name: &str, param_type: &str, description: &str) -> ParamDoc {
    ParamDoc {
        name: name.to_string(),
        param_type: param_type.to_string(),
        description: description.to_string(),
    }
}

#[macro_export]
macro_rules! method_doc {
    ($name:expr, $module:expr, $sig:expr, $desc:expr) => {
        $crate::wiki::extractor::create_method_doc($name, $module, $sig, $desc)
    };
}

#[macro_export]
macro_rules! param_doc {
    ($name:expr, $type:expr, $desc:expr) => {
        $crate::wiki::extractor::create_param_doc($name, $type, $desc)
    };
}
