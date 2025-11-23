// A collection of DTOs and other (de)serializable models
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct Service {
    pub mapping:    String,
    pub url:        String
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub host:       String,
    pub port:       u16,
    pub services:   Vec<Service>,
    pub blacklist:  Vec<String>,
    pub cors:       Vec<String>
}

#[derive(Serialize)]
pub struct GenericResponse {
    pub message: String
}
