#![allow(unused)]

use std::mem::{size_of, size_of_val};
use std::rc::Rc;

/*
#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}
*/
#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Rc<Node>>,
}


fn main() {
    /*
    let mut n1 = Box::new(Node { val: 1, next: None });
    let mut n2 = Box::new(Node { val: 2, next: None });
    println!("{n1:p} {n1:?}");
    println!("{n2:p} {n2:?}");

    n1.next = Some(n2);
    println!("{n1:p} {n1:?}");
    */
    // let mut n1 = Rc::new(Node { val: 1, next: None });
    let mut n1 = Node { val: 1, next: None };
    let mut n4 = Node { val: 1, next: None };
    let mut n2 = Rc::new(Node { val: 2, next: None });
    let mut n3 = Rc::new(Node { val: 3, next: None });
    println!("n1 {:p} {n1:?}", &n1);
    println!("n2 {n2:p} {n2:?}");

    println!("{}", size_of::<Rc<Node>>());
    println!("{}", size_of_val(&n2));
    println!("StrongCount: {}", Rc::strong_count(&n2));

    n1.next = Some(Rc::clone(&n2));
    n4.next = Some(Rc::clone(&n2));
    // println!("n1 {:p} {n1:?}", &n1);
    println!("n2 {n2:p} {n2:?}");
    println!("StrongCount: {}", Rc::strong_count(&n2));
}
