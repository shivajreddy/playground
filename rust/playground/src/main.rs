#![allow(unused)]
use std::{cell::RefCell, fmt::Display, rc::Rc};

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
            // self.tail.unwrap().borrow_mut().next = Some(node.clone());   // why should i use clone here?
            let prev_tail = self.tail.clone().unwrap();
            prev_tail.borrow_mut().next = Some(node.clone()); // why should i use clone here?
            self.tail = Some(node);
            self.size += 1;
        }
    }

    fn push() {}
}

fn print_lined_list<T>(linked_list: LinkedList<T>)
where
    T: Display,
{
    // get the head
    if linked_list.size == 0 {
        println!("EMPTY LIST 😢");
    } else {
        let mut curr = linked_list.head;
        while let Some(node) = curr {
            print!("{} -> ", node.borrow().val);
            curr = node.borrow().next.clone();
        }
        println!("None");
    }
}

fn main() {
    // create a linked list from vector
    let mut linked_list = LinkedList::<i32>::new();
    // print_lined_list(linked_list);

    let v = vec![10, 20, 30, 40, 50];
    for num in v.iter() {
        let new_node = Rc::new(RefCell::new(ListNode {
            val: *num,
            next: None,
        }));
        linked_list.push_node(new_node);
    }
    print_lined_list(linked_list);
}
