use crate::wiki::{WikiExport, ProgrammingLanguage};

pub struct WikiViewer {
    expanded_modules: Vec<bool>,
    expanded_methods: Vec<Vec<bool>>,
    wiki_data: Option<WikiExport>,
    selected_language: ProgrammingLanguage,
    search_query: String,
}

impl WikiViewer {
    pub fn builder() -> WikiViewerBuilder {
        WikiViewerBuilder::default()
    }

    pub fn new(wiki_data: WikiExport) -> Self {
        let num_modules = wiki_data.modules.len();
        let expanded_methods = wiki_data.modules.iter()
            .map(|m| vec![false; m.methods.len()])
            .collect();

        Self {
            expanded_modules: vec![false; num_modules],
            expanded_methods,
            wiki_data: Some(wiki_data),
            selected_language: ProgrammingLanguage::Python,
            search_query: String::new(),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        if let Some(wiki) = &self.wiki_data {
            ui.heading(format!("{} v{} - API Documentation", wiki.framework_name, wiki.version));
            
            ui.horizontal(|ui| {
                ui.label("🔍");
                ui.text_edit_singleline(&mut self.search_query);
                if ui.button("🗑️").clicked() {
                    self.search_query.clear();
                }
            });
            
            ui.horizontal(|ui| {
                ui.label("Language:");
                for lang in ProgrammingLanguage::all() {
                    if ui.selectable_value(&mut self.selected_language, lang.clone(), lang.name()).clicked() {
                        self.selected_language = lang;
                    }
                }
            });
            ui.separator();

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for (mod_idx, module) in self.wiki_data.as_ref().unwrap().modules.clone().iter().enumerate() {
                        let module_matches = module.name.to_lowercase().contains(&self.search_query.to_lowercase());
                        let methods_match: Vec<bool> = module.methods.iter()
                            .map(|m| m.name.to_lowercase().contains(&self.search_query.to_lowercase())
                                || m.description.to_lowercase().contains(&self.search_query.to_lowercase()))
                            .collect();
                        let has_matches = module_matches || methods_match.iter().any(|&m| m);
                        
                        if self.search_query.is_empty() || has_matches {
                            self.render_module(ui, mod_idx, module, &methods_match);
                        }
                    }
                });
        }
    }

    fn render_module(&mut self, ui: &mut egui::Ui, mod_idx: usize, module: &crate::wiki::ModuleDoc, methods_match: &[bool]) {
        let is_expanded = mod_idx < self.expanded_modules.len() && self.expanded_modules[mod_idx];

        ui.horizontal(|ui| {
            if ui.selectable_label(is_expanded, format!("📦 {}", module.name)).clicked() {
                if mod_idx < self.expanded_modules.len() {
                    self.expanded_modules[mod_idx] = !self.expanded_modules[mod_idx];
                }
            }
        });

        if is_expanded {
            ui.indent("module_content", |ui| {
                ui.label(&module.description);
                ui.separator();

                for (method_idx, method) in module.methods.iter().enumerate() {
                    if self.search_query.is_empty() || (method_idx < methods_match.len() && methods_match[method_idx]) {
                        self.render_method(ui, mod_idx, method_idx, method);
                    }
                }
            });
        }
    }

    fn render_method(&mut self, ui: &mut egui::Ui, mod_idx: usize, method_idx: usize, method: &crate::wiki::MethodDoc) {
        let is_expanded = mod_idx < self.expanded_methods.len()
            && method_idx < self.expanded_methods[mod_idx].len()
            && self.expanded_methods[mod_idx][method_idx];

        ui.horizontal(|ui| {
            if ui.selectable_label(is_expanded, format!("🔧 {}", method.name)).clicked() {
                if mod_idx < self.expanded_methods.len() && method_idx < self.expanded_methods[mod_idx].len() {
                    self.expanded_methods[mod_idx][method_idx] = !self.expanded_methods[mod_idx][method_idx];
                }
            }
        });

        if is_expanded {
            ui.indent("method_content", |ui| {
                let sig = match &self.selected_language {
                    ProgrammingLanguage::Python => &method.python_signature,
                    ProgrammingLanguage::CSharp => &method.csharp_signature,
                    ProgrammingLanguage::Cpp => &method.cpp_signature,
                    ProgrammingLanguage::Rust => &method.rust_signature,
                };
                ui.monospace(sig);

                ui.label(&method.description);

                if !method.parameters.is_empty() {
                    ui.label("Parameters:");
                    ui.indent("params", |ui| {
                        for param in &method.parameters {
                            ui.label(format!("• {} ({}): {}", param.name, param.param_type, param.description));
                        }
                    });
                }

                if let Some(ret) = &method.returns {
                    ui.label(format!("Returns: {}", ret));
                }

                if !method.examples.is_empty() {
                    ui.label("Examples:");
                    ui.indent("examples", |ui| {
                        for example in &method.examples {
                            let code = example.get(&self.selected_language);
                            ui.monospace(code);
                        }
                    });
                }

                ui.separator();
            });
        }
    }
}

pub struct WikiViewerBuilder {
    wiki_data: Option<WikiExport>,
}

impl Default for WikiViewerBuilder {
    fn default() -> Self {
        Self { wiki_data: None }
    }
}

impl WikiViewerBuilder {
    pub fn with_wiki(mut self, wiki: WikiExport) -> Self {
        self.wiki_data = Some(wiki);
        self
    }

    pub fn build(self) -> WikiViewer {
        if let Some(wiki) = self.wiki_data {
            WikiViewer::new(wiki)
        } else {
            WikiViewer {
                expanded_modules: Vec::new(),
                expanded_methods: Vec::new(),
                wiki_data: None,
                selected_language: ProgrammingLanguage::Python,
                search_query: String::new(),
            }
        }
    }
}
