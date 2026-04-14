use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

fn main() {
    let start_time = Instant::now(); // hitung waktu
    let (tx, rx) = mpsc::channel();
    let reader_thread = thread::spawn(move || {
    let path = "Product-Sales-Region.csv";
    let file = File::open(path).expect("Gagal membuka file");
    let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(content) = line {
                tx.send(content).unwrap();
            }
        }
    });

    let display_thread = thread::spawn(move || {
        let mut count = 0;
        for _received in rx {
            println!("{}", _received);
            count += 1;
        }
        println!("Total baris diproses: {}", count);
    });

    reader_thread.join().unwrap();
    display_thread.join().unwrap();
    let duration = start_time.elapsed(); // hitung waktu
    println!("Waktu eksekusi (2-Thread): {:?}\n", duration);
}