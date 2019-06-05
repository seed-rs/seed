use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMessageRequestBody {
    pub text: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMessageResponseBody {
    pub ordinal_number: u32,
    pub text: String,
}
