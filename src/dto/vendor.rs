use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePasswordInput {
    pub password: String,
    pub confirm_password: String
}
