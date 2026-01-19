use serde::{Serialize, Deserialize};
use super::language::CodeExample;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MethodDoc {
    pub name: String,
    pub module: String,
    pub description: String,
    pub parameters: Vec<ParamDoc>,
    pub returns: Option<String>,
    pub examples: Vec<CodeExample>,
    pub since_version: Option<String>,
    pub deprecated: bool,
    pub python_signature: String,
    pub csharp_signature: String,
    pub cpp_signature: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParamDoc {
    pub name: String,
    pub param_type: String,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModuleDoc {
    pub name: String,
    pub description: String,
    pub methods: Vec<MethodDoc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WikiExport {
    pub framework_name: String,
    pub version: String,
    pub modules: Vec<ModuleDoc>,
}

impl WikiExport {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            framework_name: name.to_string(),
            version: version.to_string(),
            modules: Vec::new(),
        }
    }

    pub fn add_module(&mut self, module: ModuleDoc) {
        self.modules.push(module);
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn to_markdown(&self, lang: &super::language::ProgrammingLanguage) -> String {
        let mut md = format!("# {} API Documentation\n\nVersion: {}\nLanguage: {}\n\n", self.framework_name, self.version, lang.name());
        
        for module in &self.modules {
            md.push_str(&format!("## {}\n\n{}\n\n", module.name, module.description));
            
            for method in &module.methods {
                md.push_str(&format!("### `{}`\n\n", method.name));
                
                let signature = match lang {
                    super::language::ProgrammingLanguage::Python => &method.python_signature,
                    super::language::ProgrammingLanguage::CSharp => &method.csharp_signature,
                    super::language::ProgrammingLanguage::Cpp => &method.cpp_signature,
                };
                
                md.push_str(&format!("**Signature:**\n```{}\n{}\n```\n\n", lang.file_extension(), signature));
                md.push_str(&format!("{}\n\n", method.description));
                
                if !method.parameters.is_empty() {
                    md.push_str("**Parameters:**\n\n");
                    for param in &method.parameters {
                        md.push_str(&format!("- `{}` (`{}`): {}\n", param.name, param.param_type, param.description));
                    }
                    md.push_str("\n");
                }
                
                if let Some(ret) = &method.returns {
                    md.push_str(&format!("**Returns:** `{}`\n\n", ret));
                }
                
                if !method.examples.is_empty() {
                    md.push_str("**Examples:**\n\n");
                    for example in &method.examples {
                        md.push_str(&format!("```{}\n{}\n```\n\n", lang.file_extension(), example.get(lang)));
                    }
                }
                
                if let Some(version) = &method.since_version {
                    md.push_str(&format!("**Since:** {}\n\n", version));
                }
                
                if method.deprecated {
                    md.push_str("**⚠️ DEPRECATED**\n\n");
                }
            }
        }
        
        md
    }

    pub fn to_html(&self, lang: &super::language::ProgrammingLanguage) -> String {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"UTF-8\">\n");
        html.push_str(&format!("<title>{} API - {}</title>\n", self.framework_name, lang.name()));
        html.push_str("<style>\n");
        html.push_str("* { margin: 0; padding: 0; box-sizing: border-box; }\n");
        html.push_str("body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh; padding: 2rem; }\n");
        html.push_str(".container { max-width: 1200px; margin: 0 auto; background: white; border-radius: 12px; box-shadow: 0 20px 60px rgba(0,0,0,0.3); overflow: hidden; }\n");
        html.push_str(".header { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 3rem 2rem; text-align: center; }\n");
        html.push_str(".header h1 { font-size: 2.5rem; margin-bottom: 0.5rem; }\n");
        html.push_str(".lang-badge { display: inline-block; background: rgba(255,255,255,0.2); padding: 0.5rem 1rem; border-radius: 20px; font-size: 0.9rem; margin: 0.5rem; }\n");
        html.push_str(".content { padding: 2rem; }\n");
        html.push_str(".module { margin: 2rem 0; padding: 2rem; background: #f8f9fa; border-left: 4px solid #667eea; border-radius: 4px; }\n");
        html.push_str(".module h2 { color: #667eea; font-size: 1.8rem; margin-bottom: 0.5rem; }\n");
        html.push_str(".module-desc { color: #666; font-size: 1rem; margin-bottom: 1.5rem; }\n");
        html.push_str(".method { margin: 1.5rem 0; padding: 1.5rem; background: white; border: 1px solid #e0e0e0; border-radius: 6px; }\n");
        html.push_str(".method h3 { color: #764ba2; font-size: 1.3rem; margin-bottom: 0.8rem; }\n");
        html.push_str(".method-desc { color: #333; line-height: 1.6; margin-bottom: 1rem; }\n");
        html.push_str(".signature { background: #f0f0f0; padding: 1rem; border-radius: 4px; font-family: 'Courier New', monospace; font-size: 0.9rem; overflow-x: auto; margin: 1rem 0; border-left: 3px solid #667eea; }\n");
        html.push_str(".params, .returns { margin: 1rem 0; }\n");
        html.push_str(".params h4, .returns h4 { color: #667eea; font-size: 0.95rem; margin-bottom: 0.5rem; }\n");
        html.push_str(".param-item { margin-left: 1.5rem; margin-bottom: 0.8rem; padding-left: 1rem; border-left: 2px solid #ddd; }\n");
        html.push_str(".param-name { font-weight: bold; color: #333; }\n");
        html.push_str(".param-type { color: #764ba2; font-style: italic; }\n");
        html.push_str(".param-desc { color: #666; font-size: 0.9rem; }\n");
        html.push_str(".examples { background: #f8f9fa; padding: 1rem; border-radius: 4px; margin: 1rem 0; }\n");
        html.push_str(".examples h4 { color: #667eea; margin-bottom: 0.8rem; }\n");
        html.push_str(".code-block { background: #2d2d2d; color: #f8f8f2; padding: 1rem; border-radius: 4px; font-family: 'Courier New', monospace; font-size: 0.85rem; overflow-x: auto; margin: 0.5rem 0; line-height: 1.4; }\n");
        html.push_str(".deprecated { color: #f093fb; font-weight: bold; padding: 0.5rem 1rem; background: #ffe0ff; border-radius: 4px; display: inline-block; margin-bottom: 1rem; }\n");
        html.push_str(".version { color: #999; font-size: 0.85rem; margin-top: 0.5rem; }\n");
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str("<div class=\"container\">\n");
        html.push_str(&format!("<div class=\"header\">\n<h1>{} API Documentation</h1>\n", self.framework_name));
        html.push_str(&format!("<div><span class=\"lang-badge\">📖 Version {}</span><span class=\"lang-badge\">💻 {}</span></div>\n", self.version, lang.name()));
        html.push_str("</div>\n");
        html.push_str("<div class=\"content\">\n");
        
        for module in &self.modules {
            html.push_str(&format!("<div class=\"module\">\n<h2>{}</h2>\n", module.name));
            html.push_str(&format!("<p class=\"module-desc\">{}</p>\n", module.description));
            
            for method in &module.methods {
                html.push_str("<div class=\"method\">\n");
                html.push_str(&format!("<h3>{}</h3>\n", method.name));
                
                if method.deprecated {
                    html.push_str("<div class=\"deprecated\">⚠️ DEPRECATED</div>\n");
                }
                
                html.push_str(&format!("<p class=\"method-desc\">{}</p>\n", method.description));
                
                let signature = match lang {
                    super::language::ProgrammingLanguage::Python => &method.python_signature,
                    super::language::ProgrammingLanguage::CSharp => &method.csharp_signature,
                    super::language::ProgrammingLanguage::Cpp => &method.cpp_signature,
                };
                
                html.push_str(&format!("<div class=\"signature\">{}</div>\n", html_escape(signature)));
                
                if !method.parameters.is_empty() {
                    html.push_str("<div class=\"params\">\n<h4>Parameters:</h4>\n");
                    for param in &method.parameters {
                        html.push_str(&format!("<div class=\"param-item\"><span class=\"param-name\">{}</span> <span class=\"param-type\">({})</span>: <span class=\"param-desc\">{}</span></div>\n", 
                            param.name, param.param_type, param.description));
                    }
                    html.push_str("</div>\n");
                }
                
                if let Some(ret) = &method.returns {
                    html.push_str(&format!("<div class=\"returns\"><h4>Returns:</h4><code>{}</code></div>\n", ret));
                }
                
                if !method.examples.is_empty() {
                    html.push_str("<div class=\"examples\">\n<h4>Examples:</h4>\n");
                    for example in &method.examples {
                        let code = example.get(lang);
                        html.push_str(&format!("<div class=\"code-block\">{}</div>\n", html_escape(code)));
                    }
                    html.push_str("</div>\n");
                }
                
                if let Some(version) = &method.since_version {
                    html.push_str(&format!("<p class=\"version\">Since: {}</p>\n", version));
                }
                
                html.push_str("</div>\n");
            }
            
            html.push_str("</div>\n");
        }
        
        html.push_str("</div>\n</div>\n</body>\n</html>\n");
        html
    }
}

fn html_escape(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
}
