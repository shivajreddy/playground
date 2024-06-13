#![allow(unused)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    val: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

/*
    - goal of interior mutability
        - in the interior there is unsafe, and outer it is wrapped in a safe API
*/


fn main() {
    let n1 = Rc::new(RefCell::new(Node { val: 1, prev: None, next: None }));
    let n2 = Rc::new(RefCell::new(Node { val: 2, prev: None, next: None }));

    n1.borrow_mut().next = Some(Rc::clone(&n2));
    n2.borrow_mut().val = 22;
    n1.borrow_mut().next = Some(Rc::clone(&n2));
    
    
    let mut s = "rat";
    let mut t = "car";
    
    let 
    
}

