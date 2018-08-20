extern crate chrono;
extern crate reqwest;
extern crate timer;

mod download;

use chrono::prelude::*;
use reqwest::Client;

fn main() {

    /* TODO: Make the duration and output path command line arguments */

    let timer = timer::Timer::new();

    /*
     * The reqwest Client has its own connection pool, so we'll share a
     * Client across all threads
     */
    let client = Client::new();
    let site = "http://www.jmu.edu/cgi-bin/parking_get_sign_data.cgi";

    let _guard = {
        /* Get the data every minute */
        timer.schedule_repeating(chrono::Duration::seconds(60), move || {
            let filename = format!("/home/parking-data/xml2/{}.xml",
                                   Utc::now().format("%Y-%m-%d-%H-%M-%S"));
            let data = match download::download_data(&client, site) {
                Ok(dat) => dat,
                Err(err) => {
                    /*
                     * Log an error and return from the closure. This does
                     * not return from main()
                     */
                    eprintln!("Error fetching data: {:?}", err);
                    return;
                },
            };
            match download::save_data(data, &filename) {
                Ok(_) => (),
                Err(err) => {
                    /*
                     * Log an error and return from the closure. This does
                     * not return from main()
                     */
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
