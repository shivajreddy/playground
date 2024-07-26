#![allow(unused)]

fn main() {
    // let x;
    let mut x;

    x = 42;

    println!("x:  {}", x);

    let y = &x;

    let z = &mut x;
    *z += 1;

    println!("z:  {}", z);
    println!("*z: {}", *z);

    println!("{}", *y);
    // println!("x:  {}", x);
}
