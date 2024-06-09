#![allow(unused)]

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

struct Node2<'long> {
    val: i32,
    next: Option<&'long Node2<'long>>,
}

fn main() {
    // head -> n2 -> n3 -> None
    let mut n1 = Node { val: 1, next: None };
    let mut n2 = Node { val: 2, next: None };
    let mut n3 = Node { val: 3, next: None };

    n1.next = Some(Box::new(n2));
    // n2.next = Some(Box::new(n3));

    let mut n11 = Node2 { val: 1, next: None };
    let mut n22 = Node2 { val: 2, next: None };
    let mut n33 = Node2 { val: 3, next: None };

    n11.next = Some(&n22);
    n22.next = Some(&n33);
}
