#![allow(unused)]
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    something()
}

fn something() {
    let shared_map: Rc<RefCell<HashMap<&str, i32>>> = Rc::new(RefCell::new(HashMap::new()));
    let mut map = shared_map.borrow_mut();
    map.insert("africa", 92388);
    map.insert("india", 188);

    println!("{:?}", shared_map);

    // let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    // shared_map.borrow_mut().insert("africa", 92388);
    // shared_map.borrow_mut().insert("kyoto", 11837);
    // shared_map.borrow_mut().insert("piccadilly", 11826);
    // shared_map.borrow_mut().insert("marbles", 38);
    //

    let shared_map: Rc<RefCell<HashMap<&str, i32>>> = Rc::new(RefCell::new(HashMap::new()));
    shared_map.borrow_mut().insert("africa", 92388);
    shared_map.borrow_mut().insert("kyoto", 11837);
    shared_map.borrow_mut().insert("piccadilly", 11826);
    shared_map.borrow_mut().insert("marbles", 38);
}
