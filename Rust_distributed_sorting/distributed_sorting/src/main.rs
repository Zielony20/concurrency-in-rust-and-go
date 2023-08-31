extern crate core;
extern crate crossbeam;
extern crate rand;

use core::time::Duration;
use crossbeam::thread;
use rand::Rng;
use std::convert::TryInto;
use std::env;
use std::sync::mpsc;
use std::thread::sleep;

static mut CHANNELS_R: Option<Vec<(mpsc::Sender<i32>, mpsc::Receiver<i32>)>> = None;
static mut CHANNELS_L: Option<Vec<(mpsc::Sender<i32>, mpsc::Receiver<i32>)>> = None;
static mut QUIT_CH_R: Option<Vec<(mpsc::Sender<bool>, mpsc::Receiver<bool>)>> = None;
static mut QUIT_CH_L: Option<Vec<(mpsc::Sender<bool>, mpsc::Receiver<bool>)>> = None;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Brak podanego parametru.");
        return;
    }

    let param = &args[1];
    let mut N: i32 = match param.parse() {
        Ok(val) => val,
        Err(err) => {
            println!("Błąd parsowania parametru: {}", err);
            return;
        }
    };

    let param = &args[2];
    let mut X: i32 = match param.parse() {
        Ok(val) => val,
        Err(err) => {
            println!("Błąd parsowania parametru: {}", err);
            return;
        }
    };

    unsafe {
        CHANNELS_R = Some(Vec::new());
        CHANNELS_L = Some(Vec::new());
        QUIT_CH_R = Some(Vec::new());
        QUIT_CH_L = Some(Vec::new());

        for _ in 0..=N {
            let (tx_r, rx_r) = mpsc::channel::<i32>();
            let (tx_l, rx_l) = mpsc::channel::<i32>();
            let (quit_r, quit_rx_r) = mpsc::channel::<bool>();
            let (quit_l, quit_rx_l) = mpsc::channel::<bool>();

            CHANNELS_R.as_mut().unwrap().push((tx_r, rx_r));
            CHANNELS_L.as_mut().unwrap().push((tx_l, rx_l));
            QUIT_CH_R.as_mut().unwrap().push((quit_r, quit_rx_r));
            QUIT_CH_L.as_mut().unwrap().push((quit_l, quit_rx_l));
        }
    }

    let mut arrays = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..N {
        let mut arr = Vec::new();
        for _ in 0..X {
            arr.push(rng.gen_range(0..100000));
        }
        arrays.push(arr.clone());
    }

    crossbeam::thread::scope(|scope| {
        for (index, array) in arrays.iter_mut().enumerate() {
            if index == 0 {
                scope.spawn(move |_| distributed_sort_first(index as i32, N, array));
            } else if index == (N - 1).try_into().unwrap() {
                scope.spawn(move |_| distributed_sort_last(index as i32, N, array));
            } else {
                scope.spawn(move |_| distributed_sort(index as i32, N, array));
            }
        }
    })
    .unwrap();
}
fn distributed_sort_first(process_id: i32, n: i32, values: &mut Vec<i32>) {
    unsafe {
        let right_chan_in = &CHANNELS_R.as_ref().unwrap()[(process_id + 1) as usize].0;
        let right_chan_out = &CHANNELS_L.as_ref().unwrap()[process_id as usize].1;
        let termination_ch_in = &QUIT_CH_R.as_ref().unwrap()[(process_id + 1) as usize].0;
        let termination_ch_out = &QUIT_CH_L.as_ref().unwrap()[process_id as usize].1;
        let mut lin = -10000;
        let mut already_send = false;
        let mut mx = update(values).1;
        let mut end = false;
        while !end {
            if mx > lin {
                right_chan_in.send(mx).unwrap();
                remove_element(values, mx);
                mx = update(values).1;
                lin = right_chan_out.recv().unwrap();
                values.push(lin);
                mx = update(values).1;
            } else if !already_send {
                termination_ch_in.send(true).unwrap();
                already_send = true;
                end = termination_ch_out.recv().unwrap();
            }
            match termination_ch_out.try_recv() {
                Ok(new_end) => end = new_end,
                Err(_) => continue,
            }
        }
    }
}
fn distributed_sort(process_id: i32, n: i32, values: &mut Vec<i32>) {
    unsafe {
        let right_chan_in = &CHANNELS_R.as_ref().unwrap()[(process_id + 1) as usize].0;
        let left_chan_in = &CHANNELS_L.as_ref().unwrap()[(process_id - 1) as usize].0;
        let right_chan_out = &CHANNELS_L.as_ref().unwrap()[process_id as usize].1;
        let left_chan_out = &CHANNELS_R.as_ref().unwrap()[process_id as usize].1;
        let right_termination_ch_in = &QUIT_CH_R.as_ref().unwrap()[(process_id + 1) as usize].0;
        let left_termination_ch_in = &QUIT_CH_L.as_ref().unwrap()[(process_id - 1) as usize].0;
        let right_termination_ch_out = &QUIT_CH_L.as_ref().unwrap()[process_id as usize].1;
        let left_termination_ch_out = &QUIT_CH_R.as_ref().unwrap()[process_id as usize].1;
        let mut lin = -1000;
        let mut l = 777;
        let mut mn = update(values).0;
        let mut mx = update(values).1;
        let mut left_ready_for_end = false;
        let mut already_send = false;
        let mut end = false;
        while !end {
            if mx > lin {
                right_chan_in.send(mx).unwrap();
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
                    already_send = true;
                }
            }
            match (
                left_chan_out.try_recv(),
                right_termination_ch_out.try_recv(),
                left_termination_ch_out.try_recv(),
            ) {
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
                    end = new_end;
                }
                (_, _, Ok(new_left_ready_for_end)) => {
                    left_ready_for_end = new_left_ready_for_end;
                }
                _ => continue,
            }
        }
    }
}
fn distributed_sort_last(process_id: i32, n: i32, values: &mut Vec<i32>) {
    unsafe {
        let left_chan_in = &CHANNELS_L.as_ref().unwrap()[(process_id - 1) as usize].0;
        let left_chan_out = &CHANNELS_R.as_ref().unwrap()[process_id as usize].1;
        let termination_ch_in = &QUIT_CH_L.as_ref().unwrap()[(process_id - 1) as usize].0;
        let termination_ch_out = &QUIT_CH_R.as_ref().unwrap()[process_id as usize].1;
        let mut l = 0;
        let mut mn = update(values).0;
        let mut end = false;
        while !end {
            match (left_chan_out.try_recv(), termination_ch_out.try_recv()) {
                (Ok(new_l), _) => {
                    values.push(new_l);
                    mn = update(values).0;
                    left_chan_in.send(mn).unwrap();
                    remove_element(values, mn);
                    mn = update(values).0;
                }
                (_, Ok(new_end)) => {
                    termination_ch_in.send(true).unwrap();
                    end = new_end;
                }
                _ => continue,
            }
        }
    }
}
fn remove_element(arr: &mut Vec<i32>, value: i32) {
    if let Some(index) = arr.iter().position(|&x| x == value) {
        arr.remove(index);
    }
}
fn update(arr: &mut Vec<i32>) -> (i32, i32) {
    arr.sort();
    if arr.is_empty() {
        return (0, 0);
    }
    let min_value = arr[0];
    let max_value = arr[arr.len() - 1];
    (min_value, max_value)
}
