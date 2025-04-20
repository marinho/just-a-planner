use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Phase {
    // id as a short int
    pub id: i16,
    pub name: String,
}

impl Phase {
    pub fn new(id: i16, name: String) -> Self {
        Self { id, name }
    }
}
