#![allow(unused)]
use std::{cell::RefCell, rc::Rc};

type Node<T> = Rc<RefCell<ListNode<T>>>;

struct ListNode<T> {
    val: T,
    next: Option<Node<T>>,
}

impl<T> ListNode<T> {
    fn new(val: T) -> Option<Node<T>> {
        Some(Rc::new(RefCell::new(ListNode { val, next: None })))
    }
}

struct LinkedList<T> {
    head: Option<Node<T>>,
    tail: Option<Node<T>>,
    size: usize,
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }
    fn push_node(&mut self, node: Node<T>) {
        // add node to empty list
        if self.size == 0 {
            self.head = Some(node.clone());
            self.tail = Some(node);
            self.size += 1;
        }
        // add node
        else {
            // get prev tail
            // let x = self.tail;
            let prev_tail = self.tail.clone().unwrap();
            prev_tail.borrow_mut().next = Some(node.clone());
            // self.tail.unwrap().borrow_mut().next = Some(node.clone());
            self.tail = Some(node);
            self.size += 1;
        }
    }

    fn push() {}
}

fn main() {
    println!("hi");
}
