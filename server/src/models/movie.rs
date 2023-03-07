use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MoviePartial {
    pub title: Option<String>,
    pub description: Option<String>,
}
