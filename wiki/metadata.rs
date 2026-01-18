use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MethodDoc {
    pub name: String,
    pub module: String,
    pub signature: String,
    pub description: String,
    pub parameters: Vec<ParamDoc>,
    pub returns: Option<String>,
    pub examples: Vec<String>,
    pub since_version: Option<String>,
    pub deprecated: bool,
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

    pub fn to_markdown(&self) -> String {
        let mut md = format!("# {} API Documentation\n\nVersion: {}\n\n", self.framework_name, self.version);
        
        for module in &self.modules {
            md.push_str(&format!("## {}\n\n{}\n\n", module.name, module.description));
            
            for method in &module.methods {
                md.push_str(&format!("### `{}`\n\n", method.name));
                md.push_str(&format!("**Signature:**\n```rust\n{}\n```\n\n", method.signature));
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
                        md.push_str(&format!("```rust\n{}\n```\n\n", example));
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

    pub fn to_html(&self) -> String {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str(&format!("<title>{} - API Documentation</title>\n", self.framework_name));
        html.push_str("<style>\n");
        html.push_str("body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto; margin: 2rem; background: #f5f5f5; }\n");
        html.push_str(".container { max-width: 1000px; margin: 0 auto; background: white; padding: 2rem; border-radius: 8px; }\n");
        html.push_str("h1 { border-bottom: 3px solid #667eea; padding-bottom: 0.5rem; }\n");
        html.push_str("h2 { color: #764ba2; margin-top: 2rem; }\n");
        html.push_str("h3 { color: #667eea; font-size: 1.2rem; }\n");
        html.push_str("code { background: #f0f0f0; padding: 0.2rem 0.4rem; border-radius: 4px; font-family: 'Courier New'; }\n");
        html.push_str("pre { background: #f0f0f0; padding: 1rem; border-radius: 4px; overflow-x: auto; }\n");
        html.push_str(".param { margin-left: 1rem; margin-bottom: 0.5rem; }\n");
        html.push_str(".deprecated { color: #f093fb; font-weight: bold; }\n");
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str(&format!("<div class=\"container\">\n<h1>{}</h1>\n<p>Version: {}</p>\n", self.framework_name, self.version));
        
        for module in &self.modules {
            html.push_str(&format!("<h2>{}</h2>\n", module.name));
            html.push_str(&format!("<p>{}</p>\n", module.description));
            
            for method in &module.methods {
                html.push_str(&format!("<h3><code>{}</code></h3>\n", method.name));
                html.push_str(&format!("<pre>{}</pre>\n", method.signature));
                html.push_str(&format!("<p>{}</p>\n", method.description));
                
                if !method.parameters.is_empty() {
                    html.push_str("<p><strong>Parameters:</strong></p>\n<ul>\n");
                    for param in &method.parameters {
                        html.push_str(&format!("<li class=\"param\"><code>{}</code> (<code>{}</code>): {}</li>\n", param.name, param.param_type, param.description));
                    }
                    html.push_str("</ul>\n");
                }
                
                if let Some(ret) = &method.returns {
                    html.push_str(&format!("<p><strong>Returns:</strong> <code>{}</code></p>\n", ret));
                }
                
                if !method.examples.is_empty() {
                    html.push_str("<p><strong>Examples:</strong></p>\n");
                    for example in &method.examples {
                        html.push_str(&format!("<pre>{}</pre>\n", example));
                    }
                }
                
                if method.deprecated {
                    html.push_str("<p class=\"deprecated\">⚠️ DEPRECATED</p>\n");
                }
                
                html.push_str("<hr>\n");
            }
        }
        
        html.push_str("</div>\n</body>\n</html>\n");
        html
    }
}
