use std::sync::{Arc, Mutex};
use std::thread;

pub fn run_simulation(num_rounds: usize) {
    let forks = (0..5)
        .map(|_| Arc::new(Mutex::new(())))
        .collect::<Vec<_>>();

    let mut handles = vec![];
    for i in 0..5 {
        let left = i;
        let right = (i + 1) % 5;
        let forks_clone = forks.clone();

        handles.push(thread::spawn(move || {
            let first = usize::min(left, right);
            let second = usize::max(left, right);

            for _ in 0..num_rounds {
                let _g1 = forks_clone[first].lock().unwrap();
                let _g2 = forks_clone[second].lock().unwrap();
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_philosophers() {
        run_simulation(10);
    }
}
