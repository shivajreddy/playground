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

impl Solution {}
