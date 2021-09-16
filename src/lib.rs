use std::sync::{Arc, Mutex};

use rayon::prelude::*;

fn counter(count: usize) -> usize {
    let mut current_count = 0;

    for _ in 0..count {
        current_count += 1;
    }
    current_count
}

fn multi_counter(count: usize) -> usize {
    let mut current_count = 0;
    let num_threads = 8;
    let mut handles = Vec::new();
    for _ in 0..num_threads {
        let count_per_thread = count / num_threads; 
        let handle = std::thread::spawn(move || {
            let mut local_count = 0;
            for _ in 0..count_per_thread {
                local_count += 1;
            }
            local_count
        });
        handles.push(handle);
    }
    for handle in handles {
        // join() means sit & wait
        // unwrap means break if error happened, otherwise return value
        let local_count = handle.join().unwrap();
        current_count += local_count;
    }
    current_count
}


// mutex short for Mutual exclusion, a data structure don't allow others touch the same thing at the same time
fn multi_counter_mutex(count: usize) -> usize {
    // Arc reference atomic reference count, is a special type can copy the pointer itself and count the pointer
    let current_count: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let num_threads = 8;
    let mut handles = Vec::new();
    for _ in 0..num_threads {
        let count_per_thread = count / num_threads; 
        let current_count = current_count.clone();
        let handle = std::thread::spawn(move || {
            let mut local_count: usize = 0;
            for _ in 0..count_per_thread {
                local_count += 1;
            }
            // lock() can check the lock situation and wait until it's unlocked
            let mut guard = current_count.lock().unwrap();
            *guard += local_count;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    // into_inner() will take the value and break the mutex

    let guard = current_count.lock().unwrap();
    *guard
}

fn rayon_counter(count: usize) -> usize {
    let mut total_count = 0;
    let num_threads = 8;
    let count_per_thread = count / num_threads;

    let local_counts: Vec<_> = (0..num_threads).into_par_iter().map(|_| {
        let mut local_count: usize = 0;

        for _ in 0..count_per_thread  {
            local_count += 1;
        }

        local_count
    }).collect();

    for l in local_counts {
        total_count += l;
    }

    total_count
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{counter, multi_counter, multi_counter_mutex, rayon_counter};

    #[test]
    fn it_works() {
        let count = 1_000_000_000;
        let start = Instant::now();
        let _out = counter(count);
        println!("time-single: {}ms", start.elapsed().as_millis());

        let start = Instant::now();
        for _ in 0..1000 {
            let _out = multi_counter(count);
        } 
        println!("time-multi: {}ms", start.elapsed().as_millis());

        let start = Instant::now();
        for _ in 0..1000 {
            let out = rayon_counter(count);
        } 
        println!("time-rayon: {}ms", start.elapsed().as_millis());

        let start = Instant::now();
        for _ in 0..1000 {
            let _out = multi_counter_mutex(count);
        } 
        println!("time-multi-mutex: {}ms", start.elapsed().as_millis());


        // assert_eq!(out, count)
    }
}

// can run your fn in a fastest mode
// cargo test --release -- --nocapture