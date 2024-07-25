#![allow(unused)]

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

use std::{cell::RefCell, rc::Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    val: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(val: T) -> Rc<RefCell<Node<T>>> {
        let new_node = Node {
            val,
            prev: None,
            next: None,
        };
        Rc::new(RefCell::new(new_node))
    }
}

struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn push_front(&mut self, val: T) {
        let new_node = Node::new(val);

        // let old_head = self.head.take();
        // match old_head {
        match self.head.take() {
            Some(old_head) => {
                // {
                //     let mut old_head_inside = old_head.borrow_mut();
                //     old_head_inside.prev = Some(new_node.clone());
                // }
                // {
                //     let mut new_node_inside = new_node.borrow_mut();
                //     new_node_inside.next = Some(old_head);
                // }
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);

                self.head = Some(new_node);
                self.size += 1;
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
                self.size += 1;
            }
        }
    }
}

fn print_list(list: List<i32>) {
    let mut curr = list.head;
    while let Some(rc_node) = curr {
        let v = rc_node.borrow_mut().val;
        println!("{v}");
        curr = rc_node.borrow_mut().next.clone();
    }
}

fn main() {
    let mut list = List::new();
    let v1 = vec![10, 20, 30, 40, 50];

    for num in v1.iter() {
        list.push_front(*num);
    }
    print_list(list);
}
