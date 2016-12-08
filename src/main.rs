extern crate rand;
extern crate time;

use rand::{Rng, thread_rng};
use time::{precise_time_ns};

const N: usize = 500000;

fn main() {
    let mut numbers:Vec<_> = (0..N).collect();
    thread_rng().shuffle(&mut numbers);

    let start = precise_time_ns();
    numbers.sort();
    let duration = precise_time_ns() - start;

    println!("Sorted {} ints in {} ms", N, duration / 1000000);
}
