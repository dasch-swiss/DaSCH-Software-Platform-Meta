use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Organization {
    id: String,
}

impl Organization {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}
