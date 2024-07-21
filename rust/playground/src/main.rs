fn main() {
    Solution::coin_change(vec![1, 2, 5], 100);
}

struct Solution {}

impl Solution {
    fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut memo = vec![0];

        for i in 1..amount + 1 {
            memo.push(i);
        }

        println!("memo:?");
        -1
    }
}
