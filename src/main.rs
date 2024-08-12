use std::sync::mpsc;
use threadpool::ThreadPool;
use httploadtesting::Config;

fn main() {
    let config = Config::new();
    let pool = ThreadPool::new(config.threads());

    let (tx, rx) = mpsc::channel();

    for _ in 0..config.number() {

        let config = config.clone();

        let tx = tx.clone();
        pool.execute(move || {
            // Get the status
            match config.status() {
                // Could connect but may be a page failure such as 404
                Ok(val) => {

                    // See what code it is
                    match val {
                        404 => tx.send(false).unwrap(),

                        _ => {
                            tx.send(true).unwrap()
                        },
                    };
                },

                // Error in connecting
                _ => tx.send(false).unwrap(),
            }
        });
    }

    // Receive all the connections
    let mut statuses: Vec<bool> = Vec::with_capacity(config.number());
    for _ in 0..config.number() {
        statuses.push(
            rx.recv().unwrap()
        );
    }


    let mut success_count = 0;

    for status in statuses {
        if status {
            success_count += 1;
        }
    }

    println!("Successes: {}", success_count);
    println!("Failures: {}", config.number() - success_count);
}
