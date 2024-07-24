#![allow(unused)]

use std::{cell::RefCell, rc::Rc};

/*
    [ ] Create ListNode
    [ ] Generate LinkedList from vector
    [ ] Print LinkedList
    [ ] Reverse LinkedList
    Docs say RefCell allows borrow-check done at run time, instead of compile time.
        - since Refcell borrws are dynamic, it is possible to attempt to borrow
          a value that is already mutably borrowed. When this happens, thread panics.
        - write code for this scenario where the thread panics, and also write
          code to show how borrw-check is done compile time when not using RefCell.
*/

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    val: i32,
    next: Link,
}

impl Node {
    // wrap the node object in Rc<RefCell<>> cuz this will be
    // referenced by other nodes
    fn new(val: i32) -> Rc<RefCell<Self>> {
        let new_node = Node { val, next: None };
        Rc::new(RefCell::new(new_node))
    }
}

struct LinkedList {
    head: Link,
    tail: Link,
    size: usize,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn create_from_nums(v: Vec<i32>) -> Link {
        None
    }

    fn push_new_val(&mut self, val: i32) {
        // create a node, and wrap in Rc<RefCell<>>
        let new_node = Node::new(val);

        let p = Some(new_node);
        let q = p.unwrap();

        // let mut x = q.borrow_mut();
        // x.val = 777;
        //
        // q.borrow_mut().val = 666;

        let x = self.tail.clone().unwrap();

        // let y = self.tail.take();
        //
        // match self.tail.take() {
        //     Some(node) => {}
        //     None => {}
        // }
    }

    /*
    fn push(&mut self, node: ListNode) {
        match self.head {
            None => {
                self.head = Some(Rc::new(RefCell::new(node)));
                let n = self.head.unwrap();
                Rc::clone(&n);
                let f = n.get_mut();
                // self.tail = Some(Rc::new(RefCell::new(node)));
            }
            _ => {}
        }
    }
    */

    fn pop(&self) -> Link {
        None
    }

    fn print_list(&self) {}
}

fn main() {
    // let mut n1 = Node::new(1, None);
    // let mut n2 = Node::new(2, None);

    // LinkedList
}
