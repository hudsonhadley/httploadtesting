use std::env::args;
use reqwest;
use tokio;

#[tokio::main]
async fn main() {
    let mut args = args();
    args.next();

    let path = match args.len() {
        0 => panic!("No path given"),

        1 => args.next().unwrap(),

        _ => panic!("More than 1 argument given, only one necessary"),
    };


    let status = reqwest::get(path)
        .await
        .unwrap()
        .status()
        .as_u16();

    println!("Response code: {status}");


}
