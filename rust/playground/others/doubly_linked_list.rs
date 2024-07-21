#![allow(unused)]

use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    // key: i32,
    val: i32,
    prev: Option<Rc<Node>>,
    next: Option<Rc<Node>>,
}

pub  fn ll() {
    //  1 <-> 2 <-> 3
    let mut n1 = Rc::new(Node {
        val: 1,
        prev: None,
        next: None,
    });

    let mut n2 = Rc::new(Node {
        val: 1,
        // prev: None,
        prev: Some(Rc::clone(&n1)),
        next: None,
        // next: Some(Rc::clone(&n3)),
    });

    let mut n3 = Rc::new(Node {
        val: 1,
        prev: None,
        next: None,
    });

    let x = RefCell::new(10);
}

fn print_list(head: Node) {
    let curr = &head;
}

