#![allow(unused)]
use std::mem;

// fn replace_with_84(s: &mut Box<i32>) {
// fn replace_with_84(s: &mut String) {
//     let was = *s;
//     // let was =
// }

fn main() {
    println!("---BGN---");
    let mut data = String::from("hello");

    // let s = &data;
    let s = &mut data;
    println!("{}", *s);
    // let was = *s;
    // *s = "".to_string();
    let was = std::mem::take(s);
    *s = was;

    println!("data: {}", data);

    // println!("{}", s);

    println!("---END---");

    let x = 42;
    // drop(x);

    let foobar = FooBar {
        p1: 42,
        p2: "hello there",
    };
    // println!("{}", foobar.p1);
    // mem::drop(foobar);

    let y = 84;
    let b = Bar { p1: &y };
    let f = Foo::new(&x, b);
}

#[derive(Debug)]
struct Foo<'a> {
    p1: &'a i32,
    p2: Bar<'a>,
}

impl<'a> Foo<'a> {
    fn new(p1: &'a i32, p2: Bar<'a>) -> Self {
        Foo { p1, p2 }
    }
}

impl<'a> Drop for Foo<'a> {
    fn drop(&mut self) {}
}

#[derive(Debug)]
struct Bar<'a> {
    p1: &'a i32,
}

struct FooBar<'a> {
    p1: i32,
    p2: &'a str,
}

impl<'a> Drop for FooBar<'a> {
    fn drop(&mut self) {
        println!("Droppping your item: [{} {}]", self.p1, self.p2);
    }
}
