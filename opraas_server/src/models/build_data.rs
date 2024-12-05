use serde::Deserialize;

#[derive(Deserialize)]
pub struct BuildData {
    pub name: String,
    pub email: String,
    pub message: String,
}
