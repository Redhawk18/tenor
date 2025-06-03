use serde::{Deserialize, Serialize};

use crate::Limit;

#[derive(Debug, Default)]
pub struct Parameters {
    pub client_key: String,
    pub limit: Limit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub locale: String,
    pub results: Vec<String>,
}
