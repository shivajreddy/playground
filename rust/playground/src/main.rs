fn main() {}

struct Solution {}

#[allow(unused)]
impl Solution {
    pub fn calculate(s: String) -> i32 {
        // Variables to store the state
        let mut res = 0;
        let mut curr = 0;
        let mut prev = 0;
        let mut curr_operation = '+';

        let mut idx = 0;

        // traverse through the string
        while idx < s.len() {
            let curr_char = s.chars().nth(idx).unwrap();

            // Found a space
            if curr_char == ' ' {
                continue;
            }

            // Found a digit
            if curr_char.is_digit(10) {}
        }

        -1
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
