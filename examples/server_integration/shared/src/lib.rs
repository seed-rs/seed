use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Data {
    pub val: i8,
    pub text: String,
}