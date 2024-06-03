#![allow(unused)]

use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    val: i32,
    prev: Option<Rc<Node>>,
    next: Option<Rc<Node>>,
}

fn main() {
    //  1 <-> 2
    let mut n1 = Rc::new(Node {
        val: 1,
        prev: None,
        next: None,
        // next: Some(Rc::clone(&n2)),
    });

    let mut n2 = Rc::new(Node {
        val: 1,
        prev: Some(Rc::clone(&n1)),
        next: None,
    });

    // 1 -> 2
    n1.next = Some(Rc::clone(&n2));

    // 1 <-> 2
    // n2.prev = Some(Rc::clone(&n1));
}

fn print_list(head: Node) {
    let curr = &head;

    // while let node = *curr {
    //     println!("{}", node.val);
    // }
}

/*
struct DoublyLinkedList {
    size: u32,
    head: Node,
    tail: Node,
}

impl DoublyLinkedList {
    fn new() -> Self {
        let mut head = Node {
            // key: -1,
            val: -1,
            next: &Rc::,
            prev: None,
        };
        let mut tail = Node {
            // key: -1,
            val: -1,
            next: None,
            prev: None,
        };
        head.next = Some(Rc::new(tail));

        tail.prev = Some(Rc::clone(head));

        Self {
            size: 0,
            head: Node {
                val: -1,
                next: None,
                prev: None,
            },
            tail: Node {
                val: -1,
                next: None,
                prev: None,
            },
        }
    }
}

struct LRUCache {}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {}
    }
    fn get(&self, key: i32) -> i32 {
        1
    }
    fn put(&self, key: i32, value: i32) {}
}

#[test]
fn test_1() {
    let _ = LRUCache::new(10);
}
*/
