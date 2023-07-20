use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use rand::Rng;
use std::cmp::Ordering;
use std::sync::{Arc, Mutex};

    
fn distributed_sort_first(process_id: i32, n: i32, values: &mut Vec<i32>, right_chan_in: &mpsc::Sender<i32>, right_chan_out: &mpsc::Receiver<i32>, termination_ch_in: &mpsc::Sender<bool>, termination_ch_out: &mpsc::Receiver<bool>) {
    let mut lin = -10000;
    let mut already_send = false;
    let mut mx = update(values).1;
    let mut end = false;

    while !end {
        if mx > lin {
            right_chan_in.send(mx).unwrap();
            log_to_console(&format!("{} wysłałem {}", process_id, mx), &[process_id, mx]);
            remove_element(values, mx);
            mx = update(values).1;
            lin = right_chan_out.recv().unwrap();
            values.push(lin);
            mx = update(values).1;
        } else if !already_send {
            termination_ch_in.send(true).unwrap();
            log_to_console(&format!("{} send termination to right", process_id), &[] as &[&dyn std::fmt::Debug]);
            already_send = true;
            end = termination_ch_out.recv().unwrap();
        }
        thread::sleep(Duration::from_secs(1));

        match termination_ch_out.try_recv() {
            Ok(new_end) => end = new_end,
            Err(_) => continue,
        }
    }

    println!("{}: Wychodzę! {:?}", process_id, values);
    thread::sleep(Duration::from_millis(10));
}

 

fn distributed_sort(process_id: i32, n: i32, values: &mut Vec<i32>, right_chan_in: &mpsc::Sender<i32>, left_chan_in: &mpsc::Sender<i32>,
     right_chan_out: &mpsc::Receiver<i32>, left_chan_out: &mpsc::Receiver<i32>, right_termination_ch_in: &mpsc::Sender<bool>,
      left_termination_ch_in: &mpsc::Sender<bool>, right_termination_ch_out: &mpsc::Receiver<bool>, left_termination_ch_out: &mpsc::Receiver<bool>) {
    let mut lin = -1000;
    let mut l = 777;
    let mut mn = update(values).0;
    let mut mx = update(values).1;
    let mut left_ready_for_end = false;
    let mut already_send = false;
    let mut end = false;

    while !end {
 //       log_to_console(&format!("{} next loop", process_id), &[]);
        if mx > lin {
            right_chan_in.send(mx).unwrap();
   //         log_to_console(&format!("{} wysłałem {} i czekam", process_id, mx), &[]);
            remove_element(values, mx);
            mn = update(values).0;
            mx = update(values).1;
            lin = right_chan_out.recv().unwrap();
            values.push(lin);
            mn = update(values).0;
            mx = update(values).1;
        } else {
            if !already_send && left_ready_for_end {
                right_termination_ch_in.send(true).unwrap();
     //           log_to_console(&format!("{} send termination to right", process_id), &[]);
                already_send = true;
            }
        }
//        log_to_console(&format!("{} nasłuchuję pracuję {:?}", process_id, &values), &[process_id]);

        match (left_chan_out.try_recv(), right_termination_ch_out.try_recv(), left_termination_ch_out.try_recv()) {
            (Ok(new_l), _, _) => {
                values.push(new_l);
                mn = update(values).0;
                mx = update(values).1;
                left_chan_in.send(mn).unwrap();
                remove_element(values, mn);
                mn = update(values).0;
                mx = update(values).1;
            }
            (_, Ok(new_end), _) => {
                left_termination_ch_in.send(true).unwrap();
  //              log_to_console(&format!("{} get termination from right and send termination to left", process_id), &[]);
            }
            (_, _, Ok(new_left_ready_for_end)) => {
                
                left_ready_for_end = new_left_ready_for_end;
            }
            _ => continue,
        }
    }

    log_to_console(&format!("{}: Wychodzę! {:?}", process_id, &values), &[process_id]);
    thread::sleep(Duration::from_millis(100));
}

fn distributed_sort_last(process_id: i32, n: i32, values: &mut Vec<i32>, left_chan_in: &mpsc::Sender<i32>, left_chan_out: &mpsc::Receiver<i32>, termination_ch_in: &mpsc::Sender<bool>, termination_ch_out: &mpsc::Receiver<bool>) {
    let mut l = 0;
    let mut mn = update(values).0;
    let mut end = false;

    while !end {
    //    log_to_console(&format!("{} pracuję {:?}", process_id, values), &[]);

        match (left_chan_out.try_recv(), termination_ch_out.try_recv()) {
            (Ok(new_l), _) => {
                values.push(new_l);
                mn = update(values).0;
                left_chan_in.send(mn).unwrap();
      //          log_to_console(&format!("{} wysłałem {}", process_id, mn), &[]);
                remove_element(values, mn);
                mn = update(values).0;
            }
            (_, Ok(new_end)) => {
                termination_ch_in.send(true).unwrap();
        //        log_to_console(&format!("{} get termination from left", process_id), &[]);
         //       log_to_console(&format!("{} send termination from left", process_id), &[]);
                end = new_end;
            }
            _ => continue,
        }
    }

//    log_to_console(&format!("{}: Wychodzę! {:?}", process_id, values), &[]);
    thread::sleep(Duration::from_millis(30));
}

fn main() {

    unsafe{
    const N: i32 = 100;   // Number of processes
    const X: i32 = 20;    // Number of values in each process

    let mut channels_r = Vec::new();
    let mut channels_l = Vec::new();
    let mut quit_ch_r = Vec::new();
    let mut quit_ch_l = Vec::new();

    for _ in 0..=N {
        let (tx_r, rx_r) = mpsc::channel();
        let (tx_l, rx_l) = mpsc::channel();
        let (quit_r, quit_rx_r) = mpsc::channel();
        let (quit_l, quit_rx_l) = mpsc::channel();

        channels_r.push((tx_r, rx_r));
        channels_l.push((tx_l, rx_l));
        quit_ch_r.push((quit_r,quit_rx_r));
        quit_ch_l.push((quit_l,quit_rx_l));
    }

    let mut arrays = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..N {
        let mut arr = Vec::new();
        for _ in 0..X {
            arr.push(rng.gen_range(0..100000));
        }
        arrays.push(arr);
    }

    let mut handles = Vec::new();

    let mut arrays_clone = Arc::clone(&arrays);
    
    handles.push(thread::spawn(move || {
        let arrays = arrays_clone.lock().unwrap();
        let mut values = arrays[0].lock().unwrap();
        let mut channels_r = channels_r;
        let mut channels_l = channels_l;
        let mut quit_ch_r = quit_ch_r;
        let mut quit_ch_l = quit_ch_l;
    
        distributed_sort_first(
            0,
            N as i32,
            &mut values.as_mut().unwrap(),
            &channels_r[1].0,
            &channels_l[0].1,
            &quit_ch_r[1].0,
            &quit_ch_l[0].1
        );
    }));


/*
    fn distributed_sort_first(process_id: i32, n: i32, values: &mut Vec<i32>, right_chan_in: &mpsc::Sender<i32>, right_chan_out: &mpsc::Receiver<i32>,
         termination_ch_in: &mpsc::Sender<bool>, termination_ch_out: &mpsc::Receiver<bool>) {
        fn distributed_sort(process_id: i32, n: i32, values: &mut Vec<i32>, 
            right_chan_in: &mpsc::Sender<i32>, left_chan_in: &mpsc::Sender<i32>, right_chan_out: &mpsc::Receiver<i32>, left_chan_out: &mpsc::Receiver<i32>, 
            right_termination_ch_in: &mpsc::Sender<bool>, left_termination_ch_in: &mpsc::Sender<bool>, right_termination_ch_out: &mpsc::Receiver<bool>, left_termination_ch_out: &mpsc::Receiver<bool>) {
        fn distributed_sort_last(process_id: i32, n: i32, values: &mut Vec<i32>, left_chan_in: &mpsc::Sender<i32>, left_chan_out: &mpsc::Receiver<i32>, termination_ch_in: &mpsc::Sender<bool>, termination_ch_out: &mpsc::Receiver<bool>) {
*/
    for i in 1..N-1 {
        handles.push(thread::spawn(move || {
 
        let mut arrays = &arrays_clone.lock().unwrap();
        let mut channels_r = channels_r;
        let mut channels_l = channels_l;
        let mut quit_ch_r = quit_ch_r;
        let mut quit_ch_l = quit_ch_l;

            distributed_sort(
                i as i32, 
                N as i32, 
                &mut arrays[i as usize], 
                &channels_r[(i + 1) as usize].0, 
                &channels_l[(i - 1) as usize].0, 
                &channels_l[i as usize].1, 
                &channels_r[i as usize].1, 
                &quit_ch_r[(i + 1) as usize].0, 
                &quit_ch_l[(i - 1) as usize].0, 
                &quit_ch_l[i as usize].1, 
                &quit_ch_r[i as usize].1);

        }));
    }

    handles.push(thread::spawn(move || {

        let mut arrays = &arrays_clone.lock().unwrap();
        let mut channels_r = channels_r;
        let mut channels_l = channels_l;
        let mut quit_ch_r = quit_ch_r;
        let mut quit_ch_l = quit_ch_l;

        distributed_sort_last(
            (N - 1) as i32,
            N as i32,
            &mut arrays[(N - 1) as usize],
            &channels_l[(N - 2) as usize].0,
            &channels_r[(N - 1) as usize].1,
            &quit_ch_l[(N - 2) as usize].0,
            &quit_ch_r[(N - 1) as usize].1
        );
    }));

    for handle in handles {
        handle.join().expect("Oops! The child thread panicked.");
    }

    for arr in &*arrays {
        println!("{:?}", arr);
    }
}


}

fn remove_element(arr: &mut Vec<i32>, value: i32) {
    if let Some(index) = arr.iter().position(|&x| x == value) {
        arr.remove(index);
    }
}

fn update(arr: &Vec<i32>) -> (i32, i32) {
    let mut sorted_arr = arr.clone();
    sorted_arr.sort();
    if let Some(&min_value) = sorted_arr.first() {
        if let Some(&max_value) = sorted_arr.last() {
            return (min_value, max_value);
        }
    }
    (0, 0)
}

fn log_to_console(message: &str, args: &[impl std::fmt::Debug]) {
    if false {
        println!("# {} {:?}", message, args);
    }
}