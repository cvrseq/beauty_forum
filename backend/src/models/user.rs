use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct User {
    pub phone: String,
    pub provider: String,
    pub confirmation_code: String,
    pub confirmed: bool,
}
