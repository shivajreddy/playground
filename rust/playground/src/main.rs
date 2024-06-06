// mod doubly_linked_list;
// mod trie;
#![allow(unused)]

struct Node {
    val: i32,
    next: Option<Box<Node>>
}


fn main() {
    let a  = Box::new(10);
    let b: Box<[i32; 3]> = Box::new([10, 20, 30]);
    let c = b[2];

    println!("a={a} b ={b:?} c={c}");
    println!("a={a:p} b ={b:p} c={:p}", &c as *const i32);
    println!("a={a:p} b ={b:p} c={:p}", &c as *const i32);
}
