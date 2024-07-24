#![allow(unused)]

use std::cell::Cell;
/*
    [ ] Create ListNode
    [ ] Generate LinkedList from vector
    [ ] Print LinkedList
    [ ] Reverse LinkedList
*/

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32, next: Option<Box<Node>>) -> Node {
        Node { val, next: None }
    }
}

fn main() {
    let mut root: (i32, Option<i32>) = (1, None);
    let mut n2: (i32, Option<i32>) = (2, None);
}
