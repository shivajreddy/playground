#![allow(unused)]
/*
use tokio::time::{sleep, Duration};

async fn something() {
    println!("Task 1: Starting");
    sleep(Duration::from_secs(2)).await;
    println!("Task 1: Finished");

    println!("Task 2: Starting");
    sleep(Duration::from_secs(2)).await;
    println!("Task 2: Finished");
}
*/

fn main() {
    assert_eq!(4, Solution::total_combinations(12, vec![2, 3, 7]));
    assert_eq!(1, Solution::total_combinations(2, vec![2, 3, 7]));
    assert_eq!(1, Solution::total_combinations(30, vec![2, 3, 7]));
}

struct Solution {}

impl Solution {
    fn total_combinations(final_score: i32, scores: Vec<i32>) -> i32 {
        let r = scores.len();
        let c = final_score as usize;

        // create a grid
        let row = Vec::<i32>::with_capacity(c);

        let mut grid: Vec<Vec<i32>> = vec![];
        for _ in (0..r) {
            let mut row = vec![0; c + 1];
            row[0] = 1;
            grid.push(row);
        }
        // println!("{grid:?}");

        for r_idx in (0..r) {
            for c_idx in (1..c + 1) {
                let with_out_curr = if r_idx > 0 { grid[r_idx - 1][c_idx] } else { 0 };
                let target_col = c_idx as i32 - scores[r_idx];
                let with_curr = if target_col >= 0 {
                    grid[r_idx][target_col as usize]
                } else {
                    0
                };
                grid[r_idx][c_idx] = with_out_curr + with_curr;
            }
        }
        // println!("{grid:?}");

        let res = *grid.last().unwrap().last().unwrap();
        println!("{res}");
        res
    }
}
