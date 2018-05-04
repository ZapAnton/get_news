extern crate reqwest;

use std::{fs::File, io::{BufRead, BufReader}};

fn get_rss_addresses() -> Result<Vec<String>, std::io::Error> {
    let rss_file_path = "config/rss.txt";

    let rss_file = File::open(&rss_file_path)?;

    let rss_addrs = BufReader::new(rss_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    Ok(rss_addrs)
}

fn get_news() {
    let rss_addrs = get_rss_addresses();

    println!("{:?}", rss_addrs);
}

fn main() {
    get_news();
}
