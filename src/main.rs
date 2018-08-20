extern crate chrono;
extern crate reqwest;
extern crate timer;

mod download;

use chrono::prelude::*;
use reqwest::Client;

fn main() {
    let timer = timer::Timer::new();

    let client = Client::new();
    let site = "http://www.jmu.edu/cgi-bin/parking_get_sign_data.cgi";

    let _guard = {
        timer.schedule_repeating(chrono::Duration::seconds(60), move || {
            let filename = format!("/home/parking-data/xml2/{}.xml",
                                   Utc::now().format("%Y-%m-%d-%H-%M-%S"));
            let data = match download::download_data(&client, site) {
                Ok(dat) => dat,
                Err(err) => {
                    eprintln!("Error fetching data: {:?}", err);
                    return;
                },
            };
            match download::save_data(data, &filename) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Error writing data: {:?}", err);
                    return;
                },
            };
            println!("Wrote to {}", filename);
        })
    };

    loop {
        /* We never exit so that the schedule will run forever */
    }

}
