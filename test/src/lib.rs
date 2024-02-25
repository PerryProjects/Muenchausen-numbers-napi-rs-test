#![deny(clippy::all)]

use std::time::{Instant};


#[macro_use]
extern crate napi_derive;

const N: i32 = 440_000_000;

fn is_munchausen(number: i32, cache: &[i32; 10]) -> bool {
    let mut n = number;
    let mut total = 0;

    while n > 0 {
        let digit = n % 10;
        total += cache[digit as usize];
        if total > number {
            return false;
        }
        n /= 10;
    }

    number == total
}

fn get_cache() -> [i32; 10] {
    let mut cache = [0; 10]; // init. with 0s
    // cache[0] == 0
    for n in 1..=9 {
        cache[n] = (n as i32).pow(n as u32);
    }
    // println!("# {:?}", cache);
    cache
}

#[napi]
fn main() {
    let start = Instant::now();
    let cache = get_cache();

    for n in 0..N {
        if is_munchausen(n, &cache) {
            println!("{}", n);
        }
    }
    let duration = start.elapsed();
    println!("time in Rust Node Module: {:?}", duration);
}