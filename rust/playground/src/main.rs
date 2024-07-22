#![allow(unused)]
fn main() {
    assert_eq!(4, Solution::find_target_sum_ways(vec![0, 0, 1], 1));
    assert_eq!(
        256,
        Solution::find_target_sum_ways(vec![0, 0, 0, 0, 0, 0, 0, 0, 1], 1)
    );
}

struct Solution {}

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let total_sum: i32 = nums.iter().sum();

        let mut result = 0;

        let mut curr_sum = 0;

        for num in nums.iter() {
            curr_sum += num;
            if curr_sum == (total_sum + target) / 2 {
                result += 1;
            }
        }
        result
    }
}
