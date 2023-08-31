use std::sync::{Arc, Mutex};
use std::thread;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Brak podanego parametru.");
        return;
    }
    let param = &args[1];
    let mut NUM_WORKERS: i32 = match param.parse() {
        Ok(val) => val,
        Err(err) => {
            println!("Błąd parsowania parametru: {}", err);
            return;
        }
    };
    let param = &args[2];
    let mut NUM_ITERATIONS: i32 = match param.parse() {
        Ok(val) => val,
        Err(err) => {
            println!("Błąd parsowania parametru: {}", err);
            return;
        }
    };
    let shared_resource = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..NUM_WORKERS {
        let shared_resource = shared_resource.clone();

        let handle = thread::spawn(move || {
            for _ in 0..NUM_ITERATIONS {
                let mut data = shared_resource.lock().unwrap();
                *data += 1;
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
