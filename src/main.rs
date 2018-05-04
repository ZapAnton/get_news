extern crate quick_xml;
extern crate reqwest;

use std::{fs::File, io::{BufRead, BufReader}};
use quick_xml::{Reader, events::Event};

#[derive(Debug)]
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
    let mut rss_items: Vec<RssItem> = Vec::new();

    let mut reader = Reader::from_str(rss_xml);

    reader.trim_text(true);

    let mut buffer = Vec::new();

    let mut rss_item = RssItem::new(String::new(), String::new());

    let mut inside_item = false;

    loop {
        match reader.read_event(&mut buffer) {
            Ok(Event::Start(ref tag)) => match tag.name() {
                b"title" => {
                    if inside_item {
                        let mut buf = Vec::new();

                        match reader.read_event(&mut buf) {
                            Ok(Event::CData(ref cdata)) => {
                                rss_item.title.push_str(&reader.decode(cdata.escaped()))
                            }

                            Ok(Event::Text(ref text)) => {
                                rss_item.title.push_str(&reader.decode(text.escaped()))
                            }

                            _ => (),
                        }
                    }
                }

                b"description" => {
                    if inside_item {
                        let mut buf = Vec::new();

                        match reader.read_event(&mut buf) {
                            Ok(Event::CData(ref cdata)) => rss_item
                                .description
                                .push_str(&reader.decode(cdata.escaped())),

                            Ok(Event::Text(ref text)) => rss_item
                                .description
                                .push_str(&reader.decode(text.escaped())),

                            _ => (),
                        }
                    }
                }

                b"item" => inside_item = true,

                _ => (),
            },

            Ok(Event::End(ref tag)) => match tag.name() {
                b"item" => {
                    if inside_item {
                        rss_items.push(rss_item);

                        rss_item = RssItem::new(String::new(), String::new());

                        inside_item = false;
                    };
                }

                _ => (),
            },

            Ok(Event::Eof) => {
                break;
            }

            Err(error) => panic!(
                "Error at position {}: {:?}",
                reader.buffer_position(),
                error
            ),

            _ => (),
        }

        buffer.clear();
    }

    rss_items
}

fn fetch_rss(rss_addr: &str) -> Vec<RssItem> {
    let rss_xml = reqwest::get(rss_addr).unwrap().text().unwrap();

    get_rss_items(&rss_xml)
}

fn format_description(string: &str, trunc_index: usize, new_line_index: usize) -> String {
    let mut formated_description = String::new();

    for (idx, chr) in string.chars().enumerate() {
        if idx == trunc_index {
            return formated_description;
        }

        if (idx % new_line_index) == 0 {
            formated_description.push_str("\n\t")
        }

        if "\n\t\r".contains(chr) {
            continue;
        }

        formated_description.push(chr);
    }

    formated_description
}

fn print_news(news: &Vec<RssItem>) {
    for (i, item) in news.iter().enumerate() {
        let formated_title = format!("\x1b[96m{}) {}\x1b[0m", i + 1, item.title);

        let descr_len = item.description.len();

        let formated_description = format!(
            "\x1b[93m{}...\x1b[0m\n",
            format_description(&item.description, (descr_len * 3 / 10) as usize, 100)
        );

        println!("{}", formated_title);

        println!("{}", formated_description);
    }
}

fn get_news() {
    let rss_addrs = get_rss_addresses().unwrap();

    let mut all_news: Vec<RssItem> = Vec::new();

    for rss_addr in &rss_addrs {
        let news = fetch_rss(&rss_addr);

        all_news.extend(news);
    }

    print_news(&all_news);
}

fn main() {
    get_news();
}
