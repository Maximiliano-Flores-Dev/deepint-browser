use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct OnionSite {
    pub url: String,
    // Usamos Option para que no explote si el campo no existe
    pub title: Option<String>,
    pub timestamp: Option<String>,
    pub status: String,
    pub error: Option<String>, 
}

pub fn load_ingested_data() -> Vec<OnionSite> {
    let path = "src/crawler/data/index/master_db.json";
    let content = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&content).unwrap_or_else(|_| vec![])
}
