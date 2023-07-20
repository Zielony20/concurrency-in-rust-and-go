use std::env;
use std::thread;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Brak podanego parametru.");
        return;
    }

    let param = &args[1];
    //println!("Podany parametr: {}", param);
    let mut num_threads: usize = match param.parse() {
        Ok(val) => val,
        Err(err) => {
            println!("Błąd parsowania parametru: {}", err);
            return;
        }
    };

    let param = &args[2];
    let matrix_size: usize = match param.parse() {
        Ok(val) => val,
        Err(err) => {
            println!("Błąd parsowania parametru: {}", err);
            return;
        }
    };

    if num_threads == 0 {
        num_threads = matrix_size;
    }

    let rows_per_thread = (matrix_size as f64 / num_threads as f64).ceil() as usize; // liczba wierszy przypada na jeden wątek

    let matrix_a = initialize_matrix(matrix_size, matrix_size);
    let matrix_b = initialize_matrix(matrix_size, matrix_size);
    let _num_rows_a = matrix_a.len();
    let _num_cols_b = matrix_b[0].len();
    
    let threads: Vec<_> = (0..num_threads).map(|thread_index| {
        let start_row = (thread_index * rows_per_thread).min(matrix_size);
        let end_row = (start_row + rows_per_thread).min(matrix_size);
        //println!("{} {}",start_row,end_row);
        
        let matrix_a = matrix_a.clone();
        let matrix_b = matrix_b.clone();

        thread::spawn(move || {
            for row_offset in 0..(end_row - start_row) {
                let row = start_row + row_offset;
                multiply_rows(&matrix_a, &matrix_b, row);
            }
        })
    }).collect();

    for thread in threads {
        thread.join().unwrap();
    }

}

fn multiply_rows(matrix_a: &Vec<Vec<i32>>, matrix_b: &Vec<Vec<i32>>, row: usize) {
    let num_cols_a = matrix_a[0].len();
    let num_cols_b = matrix_b[0].len();
    let mut output_row = vec![0; num_cols_b];

    for j in 0..num_cols_b {
        for k in 0..num_cols_a {
            output_row[j] += matrix_a[row][k] * matrix_b[k][j];
        }
    }
}
fn initialize_matrix(rows: usize, cols: usize) -> Vec<Vec<i32>> {
    let mut rng = rand::thread_rng();
    (0..rows)
        .map(|_| (0..cols).map(|_| rng.gen_range(0..100)).collect())
        .collect()
}