extern crate quick_xml;
extern crate reqwest;

use std::{fs::File, io::{BufRead, BufReader}};
use quick_xml::{Reader, events::Event};

struct RssItem {
    title: String,

    description: String,
}

impl RssItem {
    fn new(title: String, description: String) -> Self {
        RssItem { title, description }
    }
}

fn get_rss_addresses() -> Result<Vec<String>, std::io::Error> {
    let rss_file_path = "config/rss.txt";

    let rss_file = File::open(&rss_file_path)?;

    let rss_addrs = BufReader::new(rss_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    Ok(rss_addrs)
}

fn get_rss_items(rss_xml: &str) -> Vec<RssItem> {
    let rss_items: Vec<RssItem> = Vec::new();

    let mut reader = Reader::from_str(rss_xml);

    reader.trim_text(true);

    let mut buffer = Vec::new();

    loop {
        match reader.read_event(&mut buffer) {
            Ok(Event::Eof) => {
                println!("END");
                break;
            }

            Err(error) => panic!(
                "Error at position {}: {:?}",
                reader.buffer_position(),
                error
            ),

            _ => (),

            Ok(Event::Text(ref text)) => {
                println!("Text: {:?}", text.unescape_and_decode(&reader).unwrap())
            }

            Ok(Event::Start(ref tag)) => {
                println!("Tag: {:?}", tag);
            }
        }

        buffer.clear();
    }

    rss_items
}

fn fetch_news(rss_addr: &str) {
    let rss_xml = reqwest::get(rss_addr).unwrap().text().unwrap();

    get_rss_items(&rss_xml);
}

fn get_news() {
    let rss_addrs = get_rss_addresses().unwrap();

    for rss_addr in &rss_addrs {
        fetch_news(&rss_addr);
    }
}

fn main() {
    get_news();
}
