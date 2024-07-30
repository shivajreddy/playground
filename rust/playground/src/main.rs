#![allow(unused)]

fn main() {
    assert_eq!(
        2,
        Solution::longest_common_subsequence("bd".to_string(), "abcd".to_string())
    );
    assert_eq!(
        2,
        Solution::longest_common_subsequence("oxcpqrsvwf".to_string(), "shmtulqrypy".to_string())
    );
    assert_eq!(
        3,
        Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string())
    );
}

struct Solution {}

use std::cmp::max;
use std::collections::HashMap;
type Map = HashMap<(i32, i32), i32>;

impl Solution {
    /* Recursion with Memoization
    pub fn longest_common_subsequence(word1: String, word2: String) -> i32 {
        let mut cache = Map::new();
        Self::helper(0, 0, &word1, &word2, &mut cache)
    }
    fn helper(i: i32, j: i32, word1: &str, word2: &str, cache: &mut Map) -> i32 {
        if i >= word1.len() as i32 {
            return 0;
        }
        if j >= word2.len() as i32 {
            return 0;
        }
        if !cache.contains_key(&(i, j)) {
            let letter_1 = word1.chars().nth(i as usize).unwrap();
            let letter_2 = word2.chars().nth(j as usize).unwrap();
            let val = if letter_1 == letter_2 {
                1 + Self::helper(i + 1, j + 1, word1, word2, cache)
            } else {
                max(
                    Self::helper(i + 1, j, word1, word2, cache),
                    Self::helper(i, j + 1, word1, word2, cache),
                )
            };
            cache.insert((i, j), val);
        }
        *cache.get(&(i, j)).unwrap()
    }
    // */

    /* Bottom Up
    pub fn longest_common_subsequence(word1: String, word2: String) -> i32 {
        // word1 -> row, word2 -> column
        // let (total_rows, total_cols) = ((word1.len() + 1) as i32, (word2.len() + 1) as i32);
        let (total_rows, total_cols) = ((word1.len() + 1), (word2.len() + 1));

        // Create the grid
        let mut grid: Vec<Vec<i32>> = vec![vec![0; total_cols]; total_rows];

        for r in (0..total_rows - 1).rev() {
            for c in (0..total_cols - 1).rev() {
                let letter_1 = word1.chars().nth(r).unwrap();
                let letter_2 = word2.chars().nth(c).unwrap();
                if letter_1 == letter_2 {
                    grid[r][c] = 1 + grid[r + 1][c + 1];
                } else {
                    grid[r][c] = max(grid[r + 1][c], grid[r][c + 1]);
                }
            }
        }
        grid[0][0]
    }
    // */

    // /* Bottom Up with Space Optimization
    #![allow(non_snake_case)]
    pub fn longest_common_subsequence(word1: String, word2: String) -> i32 {
        let small: &str;
        let large: &str;
        if word1.len() <= word2.len() {
            small = &word1;
            large = &word2;
        } else {
            small = &word2;
            large = &word1;
        }
        let N_SMALL = small.len();
        let N_LARGE = large.len();
        let mut prev_row = vec![0; N_SMALL + 1];
        for r in (0..N_LARGE).rev() {
            // println!("prev_row befr: {:?}", prev_row);
            let mut curr_row = vec![0; N_SMALL + 1];
            for c in (0..N_SMALL).rev() {
                let letter_1 = small.chars().nth(c);
                let letter_2 = large.chars().nth(r);
                if letter_1 == letter_2 {
                    curr_row[c] = 1 + prev_row[c + 1];
                } else {
                    curr_row[c] = max(prev_row[c], curr_row[c + 1]);
                }
            }
            prev_row = curr_row;
            // println!("prev_row aftr: {:?}", prev_row);
        }
        prev_row[0]
    }
    // */
}
