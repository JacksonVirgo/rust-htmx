use core::panic;

use dotenv;
use std::{env, collections::HashMap};
use std::sync::Once;

pub struct Config {
    pub port: u16,
    pub host: String
}

impl Config {
    fn new() -> Config {
        let env = dotenv::dotenv();
        match env {
            Ok(_) => {},
            Err(_) => panic!("No .env file found")
        };

        let seeking_values = ["PORT", "HOST"];
        let mut value_hashmap: HashMap<String, String> = HashMap::new();
        for (key, value) in env::vars() {
            if seeking_values.contains(&key.as_str()) {
                value_hashmap.insert(key.clone().to_uppercase(), value.clone());
            }
        }

        let port: u16;
        match value_hashmap.get("PORT") {
            Some(value) => {
                println!("PORT value found in .env file, using value of {}", value);
                match value.parse::<u16>() {
                    Ok(parsed_value) => port = parsed_value,
                    Err(_) => panic!("PORT value is not a valid u16")
                }
            },
            None => port = 8080
        }

        let host: String;
        match value_hashmap.get("HOST") {
            Some(value) => host = value.clone(),
            None => host = String::from("localhost")
        }
        
        return Config {
            port,
            host
        };
    }
}

static mut APP_CONFIG: Option<Config> = None;
static INIT: Once = Once::new();

fn init_config() {
    unsafe {
        APP_CONFIG = Some(Config::new());
    }
}

pub fn get_config() -> &'static Config {
    INIT.call_once(|| {
        init_config();
    });

    unsafe {
        APP_CONFIG.as_ref().unwrap()
    }
}
