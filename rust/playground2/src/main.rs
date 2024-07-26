use std::collections::HashMap;

fn main() {
    assert_eq!(5, Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3));
}

struct Solution {}

type Mapper = HashMap<(i32, i32), i32>;

impl Solution {
    fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut memo = Mapper::new();
        Self::helper((nums.len() - 1) as i32, target, &nums, &mut memo)
    }

    fn helper(level: i32, number: i32, nums: &Vec<i32>, memo: &mut Mapper) -> i32 {
        if memo.contains_key(&(level, number)) {
            return *memo.get(&(level, number)).unwrap();
        }
        if level == 0 {
            let mut val = 0;
            if nums[0] == number {
                val += 1;
            }
            if nums[0] == -number {
                val += 1;
            }
            memo.insert((level, number), val);
            return val;
        }
        let mut total = 0;
        total += Self::helper(level - 1, number - nums[level as usize], nums, memo);
        total += Self::helper(level - 1, number + nums[level as usize], nums, memo);
        memo.insert((level, number), total);
        total
    }
}
