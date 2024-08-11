use clap::{Arg, Command };
use reqwest::Error;

pub fn config() -> (String, usize) {
    let m = Command::new("foo")
        .arg(Arg::new("url")
            .short('u')
            .long("url")
            .required(true))
        .arg(Arg::new("number")
            .short('n')
            .default_value("10"))
        .get_matches();

    let path: &String = m.get_one("url").expect("Did not specify path");
    let number: &String = m.get_one("number").unwrap();
    let number: usize = number.parse().unwrap();

    (String::from(path), number)
}

pub async fn status(url: &str) -> Result<u16, Error> {
    let response = reqwest::get(url)
        .await;
    
    match response {
        Ok(r) => Ok(r.status().as_u16()),
        Err(e) => Err(e)
    }
}