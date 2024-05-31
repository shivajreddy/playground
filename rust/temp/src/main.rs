#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

fn main() {
    let arr = [5; 10];
    println!("{:?}", arr);
    let mut sum = 0;
    for num in arr {
        sum += num;
    }
    println!("{sum}");

    let mut x = 0;

    // naming loops
    'outer: loop {
        x += 1;
        'inner: loop {
            if x > 10 {
                continue 'outer;
            } else {
                break 'inner;
            }
        }
        break;
    }
}
