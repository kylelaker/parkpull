#[macro_use]
extern crate serde_derive;

mod config;
mod download;

use std::fs::File;
use std::thread;
use std::time;

use config::Config;

use chrono::prelude::*;
use reqwest::Client;

/// Gets the data that needs to be fetched and saves it to a file. The file
/// will be the current time in YYYY-MM-DD-HH-MM-SS format. An error message
/// will be printed on failure either when pulling or saving the data and the
/// error will not be propogated.
fn data_helper(client: &Client, site: &str, path: &str) {
    let filename = format!("{}/{}.xml", path,
                           Utc::now().format("%Y-%m-%d-%H-%M-%S"));
    let data = match download::download_data(client, site) {
        Ok(dat) => dat,
        Err(err) => {
            eprintln!("Error fetching data: {:?}", err);
            return;
        },
    };

    if let Err(err) = download::save_data(data, &filename) {
        eprintln!("Error writing data: {:?}", err);
        return;
    }

    println!("Wrote to {}", filename);
}

fn main() {
    /*
     * The reqwest Client has its own connection pool, so we'll share a
     * Client across all threads
     */
    let config_file = match File::open("/etc/parkpull/config.yml") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Unable to open config file: {:?}", err);
            std::process::exit(1);
        },
    };

    let config: Config = match serde_yaml::from_reader(config_file) {
        Ok(conf) => conf,
        Err(err) => {
            eprintln!("Unable to parse the config file: {:?}", err);
            std::process::exit(1);
        },
    };

    let client = Client::new();
    let site = config.url;
    let path = config.path;

    let worker_thread = {
        /* Get the data every minute */
        thread::spawn(move || {
            loop {
                data_helper(&client, &site, &path);
                thread::sleep(time::Duration::from_secs(60));
            }
        })
    };

    /*
     * This should literally never be finish since the worker thread loops
     * forever.
     */
    worker_thread.join().unwrap();
}
