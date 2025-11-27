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

impl Config {
    pub fn tell_about_self(&self) {
        println!("Host: {}", self.host);
        println!("Port: {}", self.port);
        println!("Services: ");
        for (k, v) in &self.services {
            println!("-> {} => {}", k, v)
        }

        println!("Blocked IPs: ");
        for ip in &self.blacklist {
            println!("-> {}", ip)
        }

        println!("CORS IPs: ");
        for ip in &self.cors {
            println!("-> {}", ip)
        }
    }
}

#[derive(Serialize)]
pub struct GenericResponse {
    pub message: String,
}
