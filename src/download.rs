use reqwest;
use reqwest::Client;

use std::io;
use std::io::Write;
use std::fs::File;

pub fn download_data(client: &Client, url: &str) -> reqwest::Result<Vec<u8>> {
    let mut response = client.get(url).send()?;
    let mut buf: Vec<u8> = vec![];
    response.copy_to(&mut buf)?;
    return Ok(buf);
}

pub fn save_data(data: Vec<u8>, filename: &str) -> io::Result<Vec<u8>> {
    let mut file = File::create(filename)?;
    file.write_all(data.as_slice())?;
    return Ok(data);
}
