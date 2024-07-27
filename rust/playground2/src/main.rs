#![allow(unused)]

fn main() {
    let mut x = 42;
    let mut y = 52;

    // foo(&x, &mut y);
    foo(&x, &mut x);
}

fn foo(input: &i32, output: &mut i32) {
    if *input == 1 {
        *output = 2;
    }
    if *input != 1 {
        *output = 3;
    }

    /*
    if *input == 1 {
        *output = 2;
    } else {
        *output = 3;
    }
    // */
}
