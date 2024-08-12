use clap::{Arg, Command };
use reqwest;
use reqwest::Error;

#[derive(Clone)]
pub struct Config {
    url: String,
    number: usize,
    threads: usize,
}

impl Config {
    pub fn new() -> Config {
        let m = Command::new("foo")
            .arg(Arg::new("url")
                .short('u')
                .long("url")
                .required(true))
            .arg(Arg::new("number")
                .short('n')
                .default_value("10"))
            .arg(Arg::new("threads")
                .short('c')
                .long("concurrent_threads")
                .default_value("1"))
            .get_matches();

        let path: &String = m.get_one("url").expect("Did not specify path");

        let number: &String = m.get_one("number").unwrap();
        let number: usize = number.parse().unwrap();

        let threads: &String = m.get_one("threads").unwrap();
        let threads: usize = threads.parse().unwrap();


        Config {
            url: String::from(path),
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

    pub fn status(&self) -> Result<u16, Error> {
        let response = reqwest::blocking::get(String::from(&self.url));
        match response {
            Ok(r) => Ok(r.status().as_u16()),
            Err(e) => Err(e)
        }
    }
}