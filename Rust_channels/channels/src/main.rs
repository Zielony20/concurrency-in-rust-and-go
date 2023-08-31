use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;
use std::env;

fn main() {
    unsafe{
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Brak podanego parametru.");
        return;
    }

    let param = &args[1];
    let mut NUM_WORKERS: usize = match param.parse() {
        Ok(val) => val,
        Err(err) => {
            println!("Błąd parsowania parametru: {}", err);
            return;
        }
    };

    let param = &args[2];
    let mut NUM_MESSAGES: usize = match param.parse() {
        Ok(val) => val,
        Err(err) => {
            println!("Błąd parsowania parametru: {}", err);
            return;
        }
    };

    let (sender, receiver) = mpsc::channel();
    let mut handles = vec![];
    let receiver_handle = thread::spawn(move || {
        for _ in 0..NUM_WORKERS*NUM_MESSAGES {
            let received_data = receiver.recv().unwrap();
        }
    });

    handles.push(receiver_handle);

    for i in 0..NUM_WORKERS {
        let sender_clone = sender.clone();
        let sender_handle = thread::spawn(move || {
            for j in 0..NUM_MESSAGES{
                sender_clone.send(i).unwrap();
            }
        });
        handles.push(sender_handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
}
