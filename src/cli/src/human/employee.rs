use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Employee {
    #[serde(default)]
    pub id: Option<String>,
    pub name: String,
    #[serde(default)]
    pub department: Option<String>,
    #[serde(default)]
    pub position: Option<String>,
    #[serde(default)]
    pub hire_date: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}
