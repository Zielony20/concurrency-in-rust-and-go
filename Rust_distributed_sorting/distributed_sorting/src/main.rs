use std::sync::mpsc;
use std::thread::sleep;
use crossbeam::thread;
use rand::Rng;
use core::time::Duration;
use std::convert::TryInto;

static mut CHANNELS_R: Option<Vec<(mpsc::Sender<i32>, mpsc::Receiver<i32>)>> = None;
static mut CHANNELS_L: Option<Vec<(mpsc::Sender<i32>, mpsc::Receiver<i32>)>> = None;
static mut QUIT_CH_R: Option<Vec<(mpsc::Sender<bool>, mpsc::Receiver<bool>)>> = None;
static mut QUIT_CH_L: Option<Vec<(mpsc::Sender<bool>, mpsc::Receiver<bool>)>> = None;

fn main() {
    const N: i32 = 32;   // Number of processes
    const X: i32 = 20;    // Number of values in each process

    // Inicjalizacja kanałów
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
        println!("{:?}", arr);
    }

    crossbeam::thread::scope(|scope| {
        for (index, array) in arrays.iter_mut().enumerate() {
            if index == 0 {
                // First element
                scope.spawn(move |_| distributed_sort_first(
                    index as i32,
                    N,
                    array,
                    ));
            } else if index == (N - 1).try_into().unwrap() {
                // Last element
                scope.spawn(move |_| distributed_sort_last(
                    index as i32,
                    N,
                    array,
                    ));
            } else {
                // Middle elements
                scope.spawn(move |_| distributed_sort(
                    index as i32,
                    N,
                    array,
                    ));
            }
        }
    })
    .unwrap();

    // print result
    for arr in arrays {
        println!("{:?}", arr);
    }
}

fn distributed_sort_first(
    process_id: i32,
    n: i32,
    values: &mut Vec<i32>,
) {
    unsafe{
        let right_chan_in = &CHANNELS_R.as_ref().unwrap()[(process_id+1) as usize].0;
        let right_chan_out = &CHANNELS_L.as_ref().unwrap()[process_id as usize].1;
        let termination_ch_in = &QUIT_CH_R.as_ref().unwrap()[(process_id+1) as usize].0;
        let termination_ch_out = &QUIT_CH_L.as_ref().unwrap()[process_id as usize].1;
    
    // Implementacja dla pierwszego przypadku
    let mut lin = -10000;
    let mut already_send = false;
    let mut mx = update(values).1;
    let mut end = false;

    while !end {
        if mx > lin {
            right_chan_in.send(mx).unwrap();
            println!("{} wysłałem {}", process_id, mx);
            remove_element(values, mx);
            mx = update(values).1;
            lin = right_chan_out.recv().unwrap();
            println!("{} Dostałem {}", process_id, lin);
            values.push(lin);
            mx = update(values).1;
        } else if !already_send {
            termination_ch_in.send(true).unwrap();
            println!("{} send termination to right", process_id);
            already_send = true;
            end = termination_ch_out.recv().unwrap();
        }
        match termination_ch_out.try_recv() {
            Ok(new_end) => end = new_end,
            Err(_) => continue,
        }
    }

    println!("{}: Wychodzę! {:?}", process_id, values);
    sleep(Duration::from_millis(10));
    }
}

fn distributed_sort(
    process_id: i32,
    n: i32,
    values: &mut Vec<i32>,
) {
    unsafe{
    let right_chan_in = &CHANNELS_R.as_ref().unwrap()[(process_id+1) as usize].0;
    let left_chan_in = &CHANNELS_L.as_ref().unwrap()[(process_id-1) as usize].0;
    let right_chan_out = &CHANNELS_L.as_ref().unwrap()[process_id as usize].1;
    let left_chan_out = &CHANNELS_R.as_ref().unwrap()[process_id as usize].1;
    let right_termination_ch_in = &QUIT_CH_R.as_ref().unwrap()[(process_id+1) as usize].0;
    let left_termination_ch_in = &QUIT_CH_L.as_ref().unwrap()[(process_id-1) as usize].0;
    let right_termination_ch_out = &QUIT_CH_L.as_ref().unwrap()[process_id as usize].1;
    let left_termination_ch_out = &QUIT_CH_R.as_ref().unwrap()[process_id as usize].1;

        
    // Implementacja dla przypadku środkowego
    let mut lin = -1000;
    let mut l = 777;
    let mut mn = update(values).0;
    let mut mx = update(values).1;
    let mut left_ready_for_end = false;
    let mut already_send = false;
    let mut end = false;

    while !end {
        //println!("DUPA DEBUG");

        if mx > lin {
            right_chan_in.send(mx).unwrap();
            println!("{} wysłałem {} i czekam", process_id, mx);
            remove_element(values, mx);
            mn = update(values).0;
            mx = update(values).1;
            lin = right_chan_out.recv().unwrap();
            println!("{} Dostałem {}", process_id, lin);
          
            values.push(lin);
            mn = update(values).0;
            mx = update(values).1;
        } else {
            if !already_send && left_ready_for_end {
                right_termination_ch_in.send(true).unwrap();
                println!("{} send termination to right", process_id);
                already_send = true;
            }
        }
        //println!("{} nasłuchuję pracuję {:?}", process_id, &values);

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
                println!("{} get termination from right and send termination to left", process_id);
                end = new_end;
            }
            (_, _, Ok(new_left_ready_for_end)) => {
                
                left_ready_for_end = new_left_ready_for_end;
            }
            _ => continue,
        }
    }

    println!("{}: Wychodzę! {:?}", process_id, &values);
    }
}


fn distributed_sort_last(
    process_id: i32,
    n: i32,
    values: &mut Vec<i32>,
) {
    unsafe{
    let left_chan_in = &CHANNELS_L.as_ref().unwrap()[(process_id-1) as usize].0;
    let left_chan_out = &CHANNELS_R.as_ref().unwrap()[process_id as usize].1;
    let termination_ch_in = &QUIT_CH_L.as_ref().unwrap()[(process_id-1) as usize].0;
    let termination_ch_out = &QUIT_CH_R.as_ref().unwrap()[process_id as usize].1;
    
    //Implementacja dla ostatniego przypadku
    let mut l = 0;
    let mut mn = update(values).0;
    let mut end = false;

    while !end {
        //println!("{} pracuję {:?}", process_id, values);

        match (left_chan_out.try_recv(), termination_ch_out.try_recv()) {
            (Ok(new_l), _) => {
                values.push(new_l);
                mn = update(values).0;
                left_chan_in.send(mn).unwrap();
                println!("{} wysłałem {}", process_id, mn);
                remove_element(values, mn);
                mn = update(values).0;
            }
            (_, Ok(new_end)) => {
                termination_ch_in.send(true).unwrap();
                println!("{} get termination from left", process_id);
                println!("{} send termination from left", process_id);
                end = new_end;
            }
            _ => continue,
        }
    }
    println!("Wychodzę (last)");
    }
}

fn remove_element(arr: &mut Vec<i32>, value: i32) {
    if let Some(index) = arr.iter().position(|&x| x == value) {
        arr.remove(index);
    }
}

/*fn update(arr: &Vec<i32>) -> (i32, i32) {
    let mut sorted_arr = arr.clone();
    sorted_arr.sort();
    if let Some(&min_value) = sorted_arr.first() {
        if let Some(&max_value) = sorted_arr.last() {
            return (min_value, max_value);
        }
    }
    (0, 0)
}*/

fn update(arr: &mut Vec<i32>) -> (i32, i32) {
    arr.sort();

    if arr.is_empty() {
        return (0, 0);
    }

    let min_value = arr[0];
    let max_value = arr[arr.len() - 1];

    (min_value, max_value)
}

fn log_to_console(message: &str) {
    if false {
        println!("# {}", message);
    }
}
