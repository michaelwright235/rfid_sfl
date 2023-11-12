#![windows_subsystem = "windows"]
#[macro_use(launch, routes, get, post, options, Responder, FromForm)] extern crate rocket;
mod config;
mod routes;
mod devices;
mod rfid_items;

use std::{net::IpAddr, str::FromStr, fs::OpenOptions};
use devices::DevicesList;
use log::*;
use simplelog::*;
use config::Config;

const LOGFILE: &str = "rfid_sfl.log";

fn prepare_server() -> Option<Config> {
    // Reading config.json
    let config_attempt = Config::get();
    if config_attempt.is_err() {
        TermLogger::init(
            LevelFilter::Debug,
            simplelog::Config::default(),
            TerminalMode::Stdout,
            ColorChoice::Auto,
        ).unwrap();
        return None;
    }

    let config = config_attempt.unwrap();
    if config.log_to_file() {
        // Init Terminal and File logger

        // Check if log file is larger then MAXLOGFILESIZE.
        // If it is then delete it
        if let Ok(log_file) = OpenOptions::new().read(true).open(LOGFILE) {
            if log_file.metadata().unwrap().len() > (config.max_log_size() as u64) * 1024 * 1024 {
                println!("Log file size is bigger than the limit. Deleting it.");
                core::mem::drop(log_file);
                match std::fs::remove_file(LOGFILE) {
                    Ok(_) => (),
                    Err(_) => println!("Unable to delete the old log file.")
                }
            } else {
                core::mem::drop(log_file);
            }
        }

        CombinedLogger::init(vec![
            WriteLogger::new(
                config.log_level(),
                simplelog::Config::default(),
                OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(LOGFILE).unwrap()
            ),
            TermLogger::new(
                config.log_level(),
                simplelog::Config::default(),
                TerminalMode::Stdout,
                ColorChoice::Auto,
            ),
        ]).unwrap();
    } else {
        // Init Terminal logger only
        TermLogger::init(
            config.log_level(),
            simplelog::Config::default(),
            TerminalMode::Stdout,
            ColorChoice::Auto,
        ).unwrap();
    }
    info!("Current config: {:?}", config);
    Some(config)
}

#[launch]
fn launch() -> _ {
    let config = match prepare_server() {
        Some(c) => c,
        None => {
            error!("Error opening config.json file! You can delete it, so the new default one will be created.");
            warn!("Using default configuration");
            Config::default()
        }
    };

    let address = IpAddr::from_str(config.address());
    if address.is_err() {
        error!("Unable to parse the IP address");
        panic!();
    }

    let rocket_config: rocket::Config = rocket::Config {
        address: address.unwrap(),
        port: config.port(),
        log_level: rocket::config::LogLevel::Off, // using our own logger
        cli_colors: !config.log_to_file(),
        workers: 1, // just for safety
        ..Default::default()
    };

    let devices_list = DevicesList::new();
    rocket::build()
        .configure(rocket_config)
        .mount("/", routes![
            crate::routes::index::handler,
        ])
        .mount("/rfid", routes![
            crate::routes::rfid_index::handler,
            crate::routes::get_devices_list::handler,
            crate::routes::get_devices_list::handler_options,
            crate::routes::get_items_list::handler,
            crate::routes::get_items_list::handler_options,
            crate::routes::write_tags::handler,
            crate::routes::write_tags::handler_options,
        ])
        .manage(devices_list)
        .manage(config)
}
