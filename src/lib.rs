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
        let local_count = handle.join().unwrap();
        current_count += local_count;
    }
    current_count
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{counter, multi_counter};

    #[test]
    fn it_works() {
        let count = 1_000_000;
        let start = Instant::now();
        let out = counter(count);
        println!("time: {}ms", start.elapsed().as_millis());

        let start = Instant::now();
        let out = multi_counter(count);
        println!("time: {}ms", start.elapsed().as_millis());

        assert_eq!(out, count);
    }
}
