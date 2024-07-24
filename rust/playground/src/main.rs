#![allow(unused)]

use std::cell::Cell;

#[derive(Debug, Clone, Copy)]
struct Node<'a> {
    val: i32,
    next: Option<&'a Node<'a>>,
}

impl<'a> Node<'a> {
    fn new(val: i32, next: Option<&'a Node<'a>>) -> Node {
        Node { val, next: None }
    }
}

fn main() {
    let mut root: (i32, Option<i32>) = (1, None);
    let mut n2: (i32, Option<i32>) = (2, None);

    // let n1 = Cell::new(Node::new(10));
    // let n2 = Cell::new(Node::new(20));
    // let n3 = Cell::new(Node::new(30));

    // let cell_1 = Cell::new(10);
    //
    // let c1_b1 = &cell_1;
    // println!("c1_b1 {:?}", c1_b1.get());
    //
    // let c1_b2 = &cell_1;
    // println!("c1_b2 {:?}", c1_b2.get());
    // c1_b2.set(11111);
    //
    // println!("c1_b1 {:?}", c1_b1.get());
}

// create a linked list
