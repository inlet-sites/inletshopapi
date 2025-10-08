use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginInput {
    email: String,
    password: String
}
