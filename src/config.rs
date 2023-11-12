use log::*;
use rocket::serde::{Serialize, Deserialize, json};
use std::io::Write;
use std::{fs::File, io::Read, net::Ipv4Addr, net::Ipv6Addr, str::FromStr};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    address: String,
    port: u16,
    log_level: String,
    log_to_file: bool,
    max_log_size: u16,
    ask_when_writing: bool
}

impl Default for Config {
    fn default() -> Self {
        Self {
            address: "127.0.0.1".to_string(),
            port: 21646,
            log_to_file: true,
            log_level: "Info".to_string(),
            max_log_size: 5,
            ask_when_writing: false,
        }
    }
}

const FILENAME: &str = "config.json";

impl Config {
    pub fn get() -> Result<Config, ()> {
        // Создание файла конфиг, если он не существует
        if !Path::new(FILENAME).exists() {
            println!("Config file config.json doesn't exit. Trying to create it...");
            let new_file = File::create(FILENAME);
            match new_file {
                Ok(mut f) => {
                    let write_result = f.write_all(Config::default_string().unwrap().as_bytes());
                    if write_result.is_ok() {
                        println!("Config file is created successfully");
                    }
                    else {
                        println!("Unable to write to config file");
                    }
                    
                },
                Err(_) => {
                    error!("Unable to create config file");
                    return Err(());
                }
            }
        }

        // Открытие файла конфиг
        let file = File::open(FILENAME);
        let mut contents = String::new();
        match file {
            Ok(mut f) => {
                let result = f.read_to_string(&mut contents);
                if result.is_err() {
                    println!("Error reading config file: {:?}", result.err().unwrap().to_string());
                    return Err(());
                }
            },
            Err(_e) => {
                //todo!("Другие варианты ошибок");
                println!("Error opening config file: {:?}", _e.to_string());
                return Err(());
            }
        };        
        
        // Парсинг JSON
        let config_json: Result<Config, json::serde_json::Error> = json::from_str(contents.as_str());

        if config_json.is_err() {
            println!("JSON error: {:?}", config_json.unwrap_err().to_string());
            return Err(());
        }

        let mut config = config_json.unwrap();

        if Ipv4Addr::from_str(&config.address).is_err() && Ipv6Addr::from_str(&config.address).is_err() {
            println!("Incorrect IP address in config.json");
            return Err(());
        }

        match config.log_level.as_str() {
            "Off" | "Error" | "Warn" | "Info" | "Debug" | "Trace" => (),
            _ => {
                config.log_level = "Info".to_string();
                println!("Field 'log_level' in config.json is incorrect. Options are: Off, Error, Warn, Info, Debug, Trace. Using 'Info'...");
            },
        }

        Ok(config)
    }

    #[allow(unused)]
    pub fn default_string() -> Result<String, ()> {
        let config = json::to_pretty_string(&Config{..Default::default()});
        match config {
            Err(_) => Err(()),
            Ok(r) => Ok(r)
        }
    }

    #[allow(unused)]
    pub fn address(&self) -> &String {
        &self.address
    }

    #[allow(unused)]
    pub fn port(&self) -> u16 {
        self.port
    }

    #[allow(unused)]
    pub fn log_to_file(&self) -> bool {
        self.log_to_file
    }

    #[allow(unused)]
    pub fn max_log_size(&self) -> u16 {
        self.max_log_size
    }

    #[allow(unused)]
    pub fn log_level(&self) -> LevelFilter {
        match self.log_level.as_str() {
            "Off" => LevelFilter::Off,
            "Error" => LevelFilter::Error,
            "Warn" => LevelFilter::Warn,
            "Debug" => LevelFilter::Debug,
            "Trace" => LevelFilter::Trace,
            _ => LevelFilter::Info,
        }
    }

    #[allow(unused)]
    pub fn ask_when_writing(&self) -> bool {
        self.ask_when_writing
    }


}
