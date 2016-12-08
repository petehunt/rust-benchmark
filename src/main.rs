extern crate rand;
extern crate time;

use rand::{Rng, thread_rng};
use time::{precise_time_ns};

fn main() {
    let mut numbers:Vec<_> = (0..100000).collect();
    thread_rng().shuffle(&mut numbers);

    let start = precise_time_ns();
    numbers.sort();
    let duration = precise_time_ns() - start;

    println!("Took {}", duration / 1000000);
}
