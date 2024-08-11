use tokio;

use httploadtesting;



#[tokio::main]
async fn main() {
    let (path, number) = httploadtesting::config();

    let mut success = true;
    for i in 0..number {
        let status = httploadtesting::status(&path).await;

        match status {
            Ok(val) if val != 404 => continue,
            _ => {
                println!("Connection {} failed", i + 1);
                success = false;
                break;
            }
        }
    }

    if success {
        println!("{} connections made successfully", number);
    }
}
