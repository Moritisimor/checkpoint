use std::collections::HashMap;

// A collection of DTOs and other (de)serializable models
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub services: HashMap<String, String>,
    pub blacklist: Vec<String>,
    pub cors: Vec<String>,
}

#[derive(Serialize)]
pub struct GenericResponse {
    pub message: String,
}
