#![allow(unused)]

use std::time::Instant;
use tokio::task;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(long_task());
    handle.await;
    println!("hello, we would reach here before long task");
}

async fn long_computation() -> u64 {
    let mut sum = 0;
    let iterations = 10_000_000_00;

    for i in 0..iterations {
        sum = (sum + 1) % 1000_000_007;
    }
    sum
}

async fn long_task() {
    let start_time = Instant::now();
    println!("Time Start:{:?}", start_time);
    let result = long_computation().await;
    println!("Time End:{:?}", Instant::now());
    let duration = start_time.elapsed();

    println!("Result:{}", result);
    println!("Time:{:?}", duration);
}
