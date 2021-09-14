fn counter(count: usize) -> usize {
    let mut current_count = 0;

    for _ in 0..count {
        current_count += 1;
    }
    current_count
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::counter;

    #[test]
    fn it_works() {
        let count = 1_000_000;
        let start = Instant::now();
        let out = counter(count);
        println!("time: {}ms", start.elapsed().as_millis());
        assert_eq!(out, count);
    }
}
