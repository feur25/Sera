use std::collections::HashMap;

const BUTTON_ORDER: &[(ButtonId, &str, &str)] = &[
    (ButtonId::Elements, "📋", "Afficher les éléments"),
    (ButtonId::Orientation, "🔄", "Changer l'orientation"),
    (ButtonId::Sort, "📊", "Trier les données"),
    (ButtonId::Mode3D, "🎲", "Mode 3D"),
    (ButtonId::Processor, "⚙", "Traitement avancé"),
    (ButtonId::Transform, "🔀", "Transformer les données"),
    (ButtonId::Wiki, "📚", "Documentation API"),
    (ButtonId::Html, "🌐", "Exporter en HTML"),
    (ButtonId::Info, "ℹ", "Information système"),
];

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ButtonId {
    ZoomIn,
    ZoomOut,
    Elements,
    Orientation,
    Sort,
    Mode3D,
    Processor,
    Transform,
    Wiki,
    Html,
    Info,
    Custom(String),
}

#[derive(Clone)]
pub struct ButtonDefinition {
    pub id: ButtonId,
    pub icon: String,
    pub text: String,
}

pub struct ButtonManager {
    buttons: Vec<ButtonDefinition>,
    states: HashMap<ButtonId, bool>,
    show_info: bool,
}

impl ButtonManager {
    pub fn builder() -> ButtonManagerBuilder {
        ButtonManagerBuilder::new()
    }

    pub fn new() -> Self {
        Self {
            buttons: Self::default_buttons(),
            states: HashMap::new(),
            show_info: false,
        }
    }

    fn default_buttons() -> Vec<ButtonDefinition> {
        BUTTON_ORDER.iter().map(|(id, icon, text)| ButtonDefinition {
            id: id.clone(),
            icon: icon.to_string(),
            text: text.to_string(),
        }).collect()
    }

    pub fn toggle_state(&mut self, id: &ButtonId) {
        let current = self.states.get(id).copied().unwrap_or(false);
        self.states.insert(id.clone(), !current);
    }

    pub fn set_state(&mut self, id: ButtonId, state: bool) {
        self.states.insert(id, state);
    }

    pub fn get_state(&self, id: &ButtonId) -> bool {
        self.states.get(id).copied().unwrap_or(false)
    }

    pub fn render_with_descriptions(&mut self, ui: &mut egui::Ui) -> HashMap<ButtonId, bool> {
        let mut clicked = HashMap::new();
        let buttons = self.buttons.clone();

        ui.horizontal_wrapped(|ui| {
            for button_def in buttons {
                if ui.button(&button_def.icon).clicked() {
                    self.toggle_state(&button_def.id);
                    clicked.insert(button_def.id, true);
                }
                
                if self.show_info {
                    ui.label(&button_def.text);
                }

                ui.separator();
            }
        });

        clicked
    }

    pub fn show_info(&mut self, value: bool) {
        self.show_info = value;
    }
}

pub struct ButtonManagerBuilder {
    buttons: Vec<ButtonDefinition>,
}

impl ButtonManagerBuilder {
    pub fn new() -> Self {
        Self {
            buttons: BUTTON_ORDER.iter().map(|(id, icon, text)| ButtonDefinition {
                id: id.clone(),
                icon: icon.to_string(),
                text: text.to_string(),
            }).collect()
        }
    }

    pub fn build(self) -> ButtonManager {
        ButtonManager {
            buttons: self.buttons,
            states: HashMap::new(),
            show_info: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_manager_builder() {
        let manager = ButtonManager::builder().build();
        assert_eq!(manager.buttons.len(), BUTTON_ORDER.len());
    }

    #[test]
    fn test_button_manager_states() {
        let mut manager = ButtonManager::new();
        
        manager.set_state(ButtonId::Elements, true);
        assert!(manager.get_state(&ButtonId::Elements));

        manager.toggle_state(&ButtonId::Elements);
        assert!(!manager.get_state(&ButtonId::Elements));
    }
}

pub struct ButtonDescription {
    pub id: ButtonId,
    pub icon: String,
    pub text: String,
}

impl ButtonDescription {
    pub fn get_description(id: &ButtonId) -> Option<(String, String)> {
        BUTTON_ORDER.iter()
            .find(|(btn_id, _, _)| btn_id == id)
            .map(|(_, icon, text)| (icon.to_string(), text.to_string()))
    }
}
