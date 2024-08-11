use reqwest;
use tokio;

use httploadtesting;



#[tokio::main]
async fn main() {
    let (path, number) = httploadtesting::config();

    let status = reqwest::get(path)
        .await
        .unwrap()
        .status()
        .as_u16();

    println!("Response code: {status}");
}
