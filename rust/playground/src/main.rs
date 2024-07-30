#![allow(unused)]

fn main() {
    assert_eq!(
        2,
        Solution::min_distance("sea".to_string(), "eat".to_string())
    );
    assert_eq!(
        4,
        Solution::min_distance("leetcode".to_string(), "etco".to_string())
    );
}

struct Solution {}

use std::cmp::min;
use std::collections::HashMap;

type Cache = HashMap<(i32, i32), i32>;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut cache = Cache::new();
        Self::helper(0, 0, &mut cache, &word1, &word2)
    }

    fn helper(i: i32, j: i32, cache: &mut Cache, word1: &str, word2: &str) -> i32 {
        if i >= word1.len() as i32 {
            return word2.len() as i32 - j;
        }
        if j >= word2.len() as i32 {
            return word1.len() as i32 - i;
        }
        if !cache.contains_key(&(i, j)) {
            let letter_1 = word1.chars().nth(i as usize).unwrap();
            let letter_2 = word2.chars().nth(j as usize).unwrap();
            let result = if letter_1 == letter_2 {
                Self::helper(i + 1, j + 1, cache, word1, word2)
            } else {
                min(
                    Self::helper(i + 1, j, cache, word1, word2),
                    Self::helper(i, j + 1, cache, word1, word2),
                ) + 1
            };
            cache.insert((i, j), result);
        }
        *cache.get(&(i, j)).unwrap()
    }
}
