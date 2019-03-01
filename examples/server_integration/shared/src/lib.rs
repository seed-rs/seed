use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Data {
    pub val: i8,
    pub text: String,
}