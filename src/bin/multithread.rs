use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Instant;

fn main() {
    let start_time = Instant::now(); // hitung waktu
    let (tx, rx) = mpsc::channel();
    let file = File::open("Product-Sales-Region.csv").expect("Gagal membuka file");
    let lines = BufReader::new(file).lines();

    // MuTex
    let shared_lines = Arc::new(Mutex::new(lines));
    let mut handles = vec![];
    let jumlah_thread_pembaca = 4; // jumlah thread bisa diubah

    for _ in 0..jumlah_thread_pembaca {
        let tx_clone = tx.clone();
        let reader_arc = Arc::clone(&shared_lines);
        let handle = thread::spawn(move || {
            loop {
                let line = {
                    let mut lock = reader_arc.lock().unwrap();
                    lock.next()
                };
                match line {
                    Some(Ok(content)) => {
                        tx_clone.send(content).unwrap();
                    }
                    _ => break,
                }
            }
        });
        handles.push(handle);
    }

    drop(tx);

    let display_thread = thread::spawn(move || {
        let mut count = 0;
        for _received in rx {
            println!("{}", _received);
            count += 1;
        }
        println!("Total baris diproses: {}", count);
    });
    for handle in handles {
        handle.join().unwrap();
    }
    display_thread.join().unwrap();
    let duration = start_time.elapsed(); // hitung waktu
    println!("Waktu eksekusi (Multi-Reader {} threads): {:?}", jumlah_thread_pembaca, duration);
}