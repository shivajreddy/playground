#![allow(unused)]

fn main() {
    assert_eq!(
        3,
        Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string())
    );
    assert_eq!(
        3,
        Solution::longest_common_subsequence("abc".to_string(), "abc".to_string())
    );
}

struct Solution {}

use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::u16;

type Map = HashMap<(u16, u16), i32>;

impl Solution {
    /*
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let word1 = text1.as_bytes();
        let word2 = text2.as_bytes();
        let (total_rows, total_cols) = (word1.len() + 1, word2.len() + 1);
        let mut grid = vec![vec![0; total_cols]; total_rows];

        for r in (0..total_rows - 1).rev() {
            for c in (0..total_cols - 1).rev() {
                if word1[r] == word2[c] {
                    grid[r][c] = 1 + grid[r + 1][c + 1];
                } else {
                    grid[r][c] = max(grid[r + 1][c], grid[r][c + 1]);
                }
            }
        }
        grid[0][0]
    }
    // */
    // /*
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let word1 = text1.as_bytes();
        let word2 = text2.as_bytes();
        let mut cache = Map::new();
        Self::rec(0, 0, &word1, &word2, &mut cache)
    }
    fn rec(i: u16, j: u16, word1: &[u8], word2: &[u8], cache: &mut Map) -> i32 {
        if i as usize >= word1.len() || j as usize >= word2.len() {
            return 0;
        }
        if let Some(val) = cache.get(&(i, j)) {
            return *val;
        }
        let mut val = 0;
        if word1[i as usize] == word2[j as usize] {
            val = 1 + Self::rec(i + 1, j + 1, word1, word2, cache);
        } else {
            val = max(
                Self::rec(i + 1, j, word1, word2, cache),
                Self::rec(i, j + 1, word1, word2, cache),
            );
        }
        cache.insert((i, j), val);
        return val;
    }

    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut cache = Map::new();
        Self::helper(0, 0, &mut cache, word1.as_bytes(), word2.as_bytes())
    }
    fn helper(i: u16, j: u16, cache: &mut Map, word1: &[u8], word2: &[u8]) -> i32 {
        if i as usize >= word1.len() {
            return (word2.len() - j as usize) as i32;
        }
        if j as usize >= word2.len() {
            return (word1.len() - i as usize) as i32;
        }
        if let Some(val) = cache.get(&(i, j)) {
            return *val;
        }
        if word1[i as usize] == word2[j as usize] {
            let val = Self::helper(i + 1, j + 1, cache, word1, word2);
            cache.insert((i, j), val);
            return val;
        } else {
            let val = 1 + min(
                Self::helper(i + 1, j, cache, word1, word2),
                Self::helper(i, j + 1, cache, word1, word2),
            );
            return val;
        }
    }

    // */
}
