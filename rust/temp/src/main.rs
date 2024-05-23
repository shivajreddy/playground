#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

fn main() {
    let c = '0';

    let s = "321";
    let mut num: u32 = 0;

    for c in s.chars() {
        if let Some(digit) = c.to_digit(10) {
            // println!("{}", digit);
            num = num * 10 + digit;
        }
    }
}
