use std::sync::mpsc;
use std::time::{SystemTime, UNIX_EPOCH};
use threadpool::ThreadPool;
use httploadtesting::Config;

fn get_time_millis() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

fn main() {
    let config = Config::new();
    let pool = ThreadPool::new(config.threads());

    let (tx_status, rx_status) = mpsc::channel();
    let (tx_time, rx_time) = mpsc::channel();

    for _ in 0..config.number() {

        let config = config.clone();

        let tx_status = tx_status.clone();
        let tx_time = tx_time.clone();


        pool.execute(move || {
            let start_time = get_time_millis();

            // Get the status
            match config.status() {
                // Could connect but may be a page failure such as 404
                Ok(val) => {

                    // See what code it is
                    match val {
                        200..300 => tx_status.send(true).unwrap(),

                        _ => tx_status.send(false).unwrap(),
                    };
                },

                // Error in connecting
                _ => tx_status.send(false).unwrap(),
            }

            // Send the time it took
            tx_time.send(get_time_millis() - start_time).unwrap();
        });
    }

    // Receive all the connections and times
    let mut statuses: Vec<bool> = Vec::with_capacity(config.number());
    let mut times: Vec<u128> = Vec::with_capacity(config.number());

    for _ in 0..config.number() {
        statuses.push(
            rx_status.recv().unwrap()
        );

        times.push(
            rx_time.recv().unwrap()
        );
    }

    // Get all the successes
    let mut success_count = 0;
    for status in statuses {
        if status {
            success_count += 1;
        }
    }

    let total_time: u128 = times.iter().sum();
    let total_time = total_time as f64 / 1000.0;

    let min_time = times.iter().min().unwrap();
    let min_time = *min_time as f64 / 1000.0;

    let max_time = times.iter().max().unwrap();
    let max_time = *max_time as f64 / 1000.0;


    println!("Results:");
    println!("  Successful Requests (2XX)...................: {}", success_count);
    println!("  Failed Requests (4XX/5XX)...................: {}", config.number() - success_count);
    println!("  Requests/second.............................: {:.2}",
             config.number() as f64/ total_time);
    println!(" ");

    println!("Total Request Time (s) (Min, Max, Mean).......: {:.2}, {:.2}, {:.2}",
             min_time, max_time, total_time / config.number() as f64);

}
