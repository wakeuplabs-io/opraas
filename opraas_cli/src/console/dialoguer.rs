use dialoguer::theme::ColorfulTheme;


pub struct Dialoguer;

impl  Dialoguer {
    pub fn new() -> Self {
        Self
    }
}

pub trait  TDialoguer {
    fn prompt(&self, message: &str) -> String;
    fn confirm(&self, message: &str) -> bool;
}

impl TDialoguer for Dialoguer {
    fn prompt(&self, message: &str) -> String {
        dialoguer::Input::with_theme(&ColorfulTheme::default()).with_prompt(message).interact_text().unwrap()
    }

    fn confirm(&self, message: &str) -> bool {
        dialoguer::Confirm::with_theme(&ColorfulTheme::default()).with_prompt(message).interact().unwrap()
    }
}