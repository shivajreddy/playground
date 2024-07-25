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
    root: Option<Node<T>>,
    tail: Option<Node<T>>,
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList {
            root: None,
            tail: None,
        }
    }
}

fn main() {
    println!("hi");
}
