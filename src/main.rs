use std::collections::HashMap;
use std::sync::mpsc;
use std::time::{SystemTime, UNIX_EPOCH};
use threadpool::ThreadPool;
use httploadtesting::Config;

fn get_time_millis() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

fn print_results(url: &String, statuses: &Vec<bool>, times: &Vec<u128>) {
    // Get all the successes
    let mut success_count = 0;
    for status in statuses {
        if *status {
            success_count += 1;
        }
    }

    let total_time: u128 = times.iter().sum();
    let total_time = total_time as f64 / 1000.0;

    let min_time = times.iter().min().unwrap();
    let min_time = *min_time as f64 / 1000.0;

    let max_time = times.iter().max().unwrap();
    let max_time = *max_time as f64 / 1000.0;


    println!("---------------------------------------------------------");
    println!(" ");
    println!("Results for `{url}`:");
    println!("  Successful Requests (2XX)...................: {}", success_count);
    println!("  Failed Requests (4XX/5XX)...................: {}", statuses.len() - success_count);
    println!("  Requests/Second.............................: {:.2}",
             statuses.len() as f64/ total_time);
    println!(" ");

    println!("Total Request Time (s) (Min, Max, Mean).......: {:.2}, {:.2}, {:.2}",
             min_time, max_time, total_time / statuses.len() as f64);

    println!(" ");
}

fn main() {
    let config = Config::new();
    let pool = ThreadPool::new(config.threads());

    let (tx_status, rx_status) = mpsc::channel();
    let (tx_time, rx_time) = mpsc::channel();

    for i in 0..config.number() {

        let config = config.clone();

        let tx_status = tx_status.clone();
        let tx_time = tx_time.clone();


        pool.execute(move || {
            let start_time = get_time_millis();

            // Get the status
            match config.status(i % config.urls_len() /* HOW DO WE INCREMENT THIS VALUE */){
                // Could connect but may be a page failure such as 404
                Ok(val) => {

                    // See what code it is
                    match val {
                        200..300 => tx_status.send(
                            (config.url(i % config.urls_len()), true)
                        ).unwrap(),

                        _ => tx_status.send(
                            (config.url(i % config.urls_len()), false)
                        ).unwrap(),
                    };
                },

                // Error in connecting
                _ => tx_status.send(
                    (config.url(i % config.urls_len()), false)
                ).unwrap(),
            }

            // Send the time it took as well as what url it was
            tx_time.send(
                (config.url(i % config.urls_len()), get_time_millis() - start_time)
            ).unwrap();
        });
    }

    // Receive all the connections and times
    let mut statuses: HashMap< String, Vec<bool> > = HashMap::with_capacity(config.number());
    let mut times: HashMap< String, Vec<u128> > = HashMap::with_capacity(config.number());

    for _ in 0..config.number() {
        let (url, status) = rx_status.recv().unwrap();


        if statuses.contains_key(&url) {
            let mut v = statuses.get(&url).unwrap().clone();
            v.push(status);

            statuses.insert(url, v);

            // If we haven't seen the url before, make a new list with the status
        } else {
            statuses.insert(url, vec![status]);
        }



        let (url, time) = rx_time.recv().unwrap();

        // If we've seen the url before, add the time to the vector
        if times.contains_key(&url) {
            let mut v = times.get(&url).unwrap().clone();
            v.push(time);

            times.insert(url, v);

            // If we haven't seen the url before, make a new list with the time
        } else {
            times.insert(url, vec![time]);
        }
    }


    for url in statuses.keys() {
        print_results(url, statuses.get(url).unwrap(), times.get(url).unwrap());
    }
}
