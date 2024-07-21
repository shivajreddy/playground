#![allow(unused)]

use std::cell::RefCell;
use std::mem::{size_of, size_of_val};
use std::ops::Deref;

fn main() {
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

    // let inner = rc1.into_inner();
    // println!("inner {:?}", &inner);
    // println!("inner {:p}", &inner);
}
