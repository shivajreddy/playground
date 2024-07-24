#![allow(unused)]
use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
};

fn main() {
    let val = RefCell::new(42);

    let mut_ref = val.borrow_mut();
    let imm_ref = val.borrow();

    println!("here");

    let mut v2 = 42;
    {
        let mut mut_ref = &mut v2;

        {
            // println!("{:p}", &v2);
        }

        *mut_ref += 1;
    }
}
