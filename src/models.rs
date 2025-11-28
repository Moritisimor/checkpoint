use std::collections::HashMap;

// A collection of DTOs and other (de)serializable models
use serde::{Deserialize, Serialize};

use crate::make_color;

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
        println!("{} {}", make_color::green("Host:"), make_color::magenta(&self.host));
        println!("{} {}", make_color::green("Port:"), make_color::magenta(&self.port.to_string()));
        println!("{}", make_color::green("Services:"));
        for (k, v) in &self.services {
            println!("{} {} {} {}", make_color::blue("->"), make_color::magenta(k), make_color::blue("=>"), make_color::magenta(v))
        }

        println!("{}", make_color::green("Blacklist:"));
        for ip in &self.blacklist {
            println!("{} {}", make_color::blue("->"), make_color::magenta(ip))
        }

        println!("{}", make_color::green("CORS:"));
        for ip in &self.cors {
            println!("{} {}", make_color::blue("->"), make_color::magenta(ip))
        }
    }
}

#[derive(Serialize)]
pub struct GenericResponse {
    pub message: String,
}
