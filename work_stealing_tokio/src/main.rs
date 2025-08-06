use std::{sync::{Arc, Mutex}, thread, time::Instant};
use std::fs::File;
use std::io::Write;
use tokio::task;
use tokio::time::Duration;

/// Simuliere CPU-Arbeit – finde die ersten `n` Primzahlen
fn compute_primes(n: usize) -> Vec<usize> {
    let mut primes = vec![];
    let mut candidate = 2;

    while primes.len() < n {
        if (2..=candidate / 2).all(|i| candidate % i != 0) {
            primes.push(candidate);
        }
        candidate += 1;
    }

    primes
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let log_data: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for i in 0..50 {
        let log_data = log_data.clone();

        let handle = task::spawn(async move {
            let thread_id = format!("{:?}", thread::current().id());
            let start = Instant::now();

            // CPU-intensive Arbeit (z. B. finde 500 kleine Primzahlen)
            let _ = compute_primes(500);

            let duration = start.elapsed().as_millis();

            let log_line = format!("{i},{thread_id},{},{duration}\n", start.elapsed().as_micros());
            let mut data = log_data.lock().unwrap();
            data.push(log_line);
        });

        handles.push(handle);
    }

    for h in handles {
        let _ = h.await;
    }

    // Schreibe Log-Datei für Visualisierung
    let mut file = File::create("thread_log.csv").unwrap();
    writeln!(file, "task,thread_id,start_us,duration_ms").unwrap();

    for entry in log_data.lock().unwrap().iter() {
        write!(file, "{}", entry).unwrap();
    }

    println!("Alle Tasks fertig. Log gespeichert in thread_log.csv");
}
