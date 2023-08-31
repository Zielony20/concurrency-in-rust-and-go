use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn channel_communication_benchmark(c: &mut Criterion) {
    c.bench_function("channel_communication", |b| {
        let num_workers = 4000;
        let num_messages = 1000;

        b.iter(|| {
            let (sender, receiver) = mpsc::channel();
            let mut handles = vec![];
            let receiver_handle = thread::spawn(move || {
                for _ in 0..num_workers * num_messages {
                    let received_data = receiver.recv().unwrap();
                }
            });

            handles.push(receiver_handle);

            for i in 0..num_workers {
                let sender_clone = sender.clone();
                let sender_handle = thread::spawn(move || {
                    for _ in 0..num_messages {
                        sender_clone.send(i).unwrap();
                    }
                });
                handles.push(sender_handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
}

criterion_group!(benches, channel_communication_benchmark);
criterion_main!(benches);
