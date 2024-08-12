use clap::{Arg, Command };
use reqwest;
use reqwest::Error;
use std::fs;

#[derive(Clone)]
pub struct Config {
    urls: Vec<String>,
    number: usize,
    threads: usize,
}

impl Config {
    pub fn new() -> Config {
        let m = Command::new("foo")

            .arg(Arg::new("url")
                .short('u')
                .long("url"))

            .arg(Arg::new("file")
                .short('f')
                .long("file"))

            .arg(Arg::new("number")
                .short('n')
                .default_value("10"))

            .arg(Arg::new("threads")
                .short('c')
                .long("concurrent_threads")
                .default_value("1"))

            .get_matches();

        let url: Option<&String> = m.get_one("url");
        let file: Option<&String> = m.get_one("file");

        let urls: Vec<String> = match (url, file) {
            (Some(_), Some(_)) => panic!("File and url provided, only one required"),
            (None, None) => panic!("No url provided"),

            // Url is provided, but no file
            (Some(url), None) => vec![url.clone()],

            // File is provided
            (None, Some(file)) => {
                // Urls should be on a new line for each one
                let file_string = fs::read_to_string(file).expect("No file found");

                let v: Vec<String> = file_string
                    .lines()
                    .map(|s| String::from(s)) // Convert from &str to String
                    .collect();

                v
            }
        };

        let number: &String = m.get_one("number").unwrap();
        let number: usize = number.parse().unwrap();

        let threads: &String = m.get_one("threads").unwrap();
        let threads: usize = threads.parse().unwrap();


        Config {
            urls,
            number,
            threads,
        }
    }

    pub fn number(&self) -> usize {
        self.number
    }

    pub fn threads(&self) -> usize {
        self.threads
    }

    pub fn urls_len(&self) -> usize {
        self.urls.len()
    }

    pub fn url(&self, index: usize) -> String {
        String::from(&self.urls[index])
    }


    pub fn status(&self, url_index: usize) -> Result<u16, Error> {
        let response = reqwest::blocking::get(
            String::from( &self.urls[url_index] )
        );

        match response {
            Ok(r) => Ok(r.status().as_u16()),
            Err(e) => Err(e)
        }
    }
}