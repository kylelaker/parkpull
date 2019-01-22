extern crate chrono;
extern crate reqwest;

mod download;

use std::thread;
use std::time;

use chrono::prelude::*;
use reqwest::Client;

/// Gets the data that needs to be fetched and saves it to a file. The file
/// will be the current time in YYYY-MM-DD-HH-MM-SS format. An error message
/// will be printed on failure either when pulling or saving the data and the
/// error will not be propogated.
fn data_helper(client: &Client, site: &str) {
    let filename = format!("/home/parking-data/xml2/{}.xml",
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
    let client = Client::new();
    let site = "http://www.jmu.edu/cgi-bin/parking_get_sign_data.cgi";

    let worker_thread = {
        /* Get the data every minute */
        thread::spawn(move || {
            loop {
                data_helper(&client, site);
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
