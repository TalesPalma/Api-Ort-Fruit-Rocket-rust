use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RecursoDTO {
    pub name: String,
    pub description: String,
}
