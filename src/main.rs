use tokio;
use httploadtesting::Config;



#[tokio::main]
async fn main() {
    let config = Config::new();

    let mut success_count = 0;

    for _ in 0..config.number() {

        match config.status().await {
            Ok(val) if val != 404 => success_count += 1,
            _ => continue,
        }
    }

    println!("Successes: {}", success_count);
    println!("Failures: {}", config.number() - success_count);
}
