#![allow(unused)]

use std::cell::RefCell;
use std::mem::{size_of, size_of_val};
use std::ops::Deref;
use std::rc::Rc;

fn mainx() {
    let b1 = Box::new(vec![10, 20, 30]);

    println!("b1_stack {:p}", b1);
    println!("b1_val {:p}", std::ptr::addr_of!(b1));
    println!("b1_val {:p}", &b1);
    println!("b1_size {} bytes", size_of_val(&b1));
    println!("b1_size {} bytes", size_of_val(b1.deref()));


    let rc1 = RefCell::new(vec![10, 20, 30]);

    println!("rc1{:?}", rc1);
    println!("rc1{:p}", &rc1);
    println!("rc1_size {}", size_of_val(&rc1));
    println!("RefCell.size {}", size_of::<RefCell<Vec<i32>>>());

    let borrow1 = rc1.borrow();
    println!("borrow1 {:?}", &borrow1);
    println!("borrow1 {:p}", &borrow1);
    println!("borrow1 {:p}", borrow1.deref());

    let borrow2 = rc1.borrow();
    println!("borrow2 {:?}", &borrow2);
    println!("borrow2 {:p}", &borrow2);
    println!("borrow2 {:p}", borrow2.deref());


    let rc = Rc::new(10);
    println!("{:p}", rc);
    println!("{:p}", &rc);

    let p = Person { name: Rc::new(String::from("shiva")), age: 10 };
    println!("{:p}", &p);
    println!("{:?}", &p);

    let x = Rc::clone(&p.name);
    println!("{:?}", &p);

    let mut num = 10;
    println!("{}", &num);
    let mut_ref = &mut num;     // ---|  scope of mutable ref
    println!("{}", mut_ref);             // ---|

    let b1 = Box::new(10);
    println!("{:?}", b1);

    let rc1 = RefCell::new(10);
    println!("{:?}", rc1);
}

#[derive(Debug)]
struct Person {
    name: Rc<String>,
    age: i32,
}

struct Node {
    val: i32,
    child: Rc<RefCell<Node>>
}

fn main() {
    let mut b1 = Box::new(10);
    
    // Compile Time Checking for Borrowing Rules
    let r1 = &mut b1;    // mutable borrow here
    println!("{}", *r1);;                    // using mutable borrow
    // let r2 = &b1;           // can't borrow again
    println!("{}", *r1);                    // using mutable borrow
    
    
    // RunTime Checking for Borrowing Rules
    let rc1 = RefCell::new(111);
    
    let r1 = RefCell::clone(&rc1);
    println!("r1: {:?}", r1);
    println!("r1: {:p}", &r1);
    // println!("&r1.borrow(): {:p}", std::ptr::addr_of!(*r1));
    // println!("{}", r1.borrow_mut());;                    // using mutable borrow
    let r2 = RefCell::clone(&rc1);
    println!("r2: {:?}", r2);
    println!("&r2: {:p}", &r2);
    println!("&r2.borrow(): {:p}", &r2.borrow());
    // println!("{}", r1.borrow_mut());;                    // using mutable borrow
    
    // Box: Single Owner
    //      Mutable+Immutable Borrows, checked @ compile time
    
    // Rc: Multiple Owners
    //     Immutable Borrows, checked @ compile time

    // RefCell: Single Owner
    //      Mutable+Immutable Borrows, checked @ RUN time
    
    // let my_box = MyBox::new(10);
    let my_box = MyBox::new("hi");
    let mbr1 = &my_box;

    // println!("{}", *my_box);
    println!("{}", my_box.deref());
    println!("{}", mbr1.val);
    // println!("{}", mbr1.deref().val);
    // println!("{:?}", mbr1);
    
    
    let s = String::from("hello there world");
    let mut iter = s.split_whitespace();
    let one = iter.next().unwrap();
    let two = iter.next().unwrap();
    let three = iter.next().unwrap_or("");
    
    let r = format!("{}-{}-{}", one, two, three);
    println!("{}", r);
    
    let x = &one[..1];
    let x = one.chars().nth(0).unwrap();
}

#[derive(Debug)]
struct MyBox<T>{
    val: T
}

impl<T> MyBox<T> {
    fn new(val: T) -> MyBox<T> {
    // fn new(val: T) -> Self {
        MyBox{val}
    }
}


// /*
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}
// */

