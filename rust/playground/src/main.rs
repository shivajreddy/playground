fn main() {}

struct Solution {}

#[allow(unused)]
impl Solution {
    pub fn calculate(s: String) -> i32 {
        println!("{}", s);
        10
    }
}

#[test]
fn test_basic_calculator2_input1() {
    let input = "3+2*2".to_string();
    let expected_output = 7;

    assert_eq!(Solution::calculate(input), expected_output);
}

#[test]
fn test_basic_calculator2_input2() {
    let input = "3+2*2".to_string();
    let expected_output = 7;

    assert_eq!(Solution::calculate(input), expected_output);
}
