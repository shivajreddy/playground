#![allow(unused)]

use std::{cell::RefCell, fmt, rc::Rc};
// use std::{cell::RefCell, fmt, rc::Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(val: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node { val, next: None }))
    }
}

struct LinkedList<T> {
    root: Rc<RefCell<Node<isize>>>,
    tail: Link<T>,
    size: usize,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        let strong_head = Rc::new(RefCell::new(Node {
            val: -1,
            next: None,
        }));
        LinkedList {
            root: strong_head,
            tail: None,
            size: 0,
        }
    }
}

fn print_list(linked_list: LinkedList<i32>) {
    let mut root = Some(linked_list.root);

    while let Some(curr) = root {
        print!("{} -> ", curr.borrow().val);
        root = curr.borrow().next.clone();
    }
    print!("None\n");
}

fn main() {
    let ll = LinkedList::<i32>::new();

    print_list(ll);

    // println!("{}", root.borrow_mut().val);
}
