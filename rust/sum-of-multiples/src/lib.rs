use std::collections::HashSet;

use std::sync::mpsc;
use std::thread;

fn generate_multiples_until(x: u32, limit: u32) -> Vec<u32> {
    let mut multiples: Vec<u32> = Vec::new();

    if x == 0 {
        return multiples;
    }

    let mut multiple: u32 = x;

    while multiple < limit {
        multiples.push(multiple);

        multiple += x;
    }

    multiples
}

fn sum_multiples(rx: mpsc::Receiver<Vec<u32>>) -> u32 {
    let mut unique_multiples: HashSet<u32> = HashSet::new();

    let mut sum: u32 = 0;

    for multiples in rx {
        for multiple in multiples {
            if unique_multiples.insert(multiple) {
                sum += multiple;
            }
        }
    }

    sum
}

pub fn sum_of_multiples(limit: u32, factors: &'static [u32]) -> u32 {
    let (tx, rx) = mpsc::channel();

    for factor in factors {
        let tx1 = mpsc::Sender::clone(&tx);

        // Generate multiples smaller than limit
        thread::spawn(move || {
            // Generate values
            let multiples = generate_multiples_until(*factor, limit);

            tx1.send(multiples).unwrap();
        });
    }

    drop(tx);

    sum_multiples(rx)
}
