use serde::{Serialize, Deserialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Data {
    pub val: i8,
    pub text: String,
}