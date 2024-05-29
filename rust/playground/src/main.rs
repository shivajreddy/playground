#![allow(dead_code)]
#![allow(unused)]
fn main() {}

struct LRUCache {}
/*

get & put must run in O(1)
get: for doing this in O(1),
    - we have to either store in a hashmap
    - tail node should be least priority item
put: for doing this in O(1)
    - cant iteratre to find a place to place the item in the collection

doubly-linked-list & hashmap of all node references.


put(a)
--------------
a(1)
--------------

put(b)
--------------
a(1) b(2)
--------------

put(c)
--------------
a(1) b(2) c(3)
--------------

put(d)
--------------
b(2) c(3) d(4)
--------------

get(c)
--------------
b(2) d(4) c(5)
--------------

get(c)
--------------
b(2) d(4) c(6)
--------------

get(b)
--------------
d(4) c(6) b(7)
--------------

get(d)
--------------
c(6) b(7) d(8)
--------------
 */

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {}
    }
    fn get(&self, key: i32) -> i32 {
        1
    }
    fn put(&self, key: i32, value: i32) {}
}

#[test]
fn test_1() {
    let _ = LRUCache::new(10);
}

struct Node {
    key: i32,
    val: i32,
    next: Option<Box<Node>>,
    prev: Option<Box<Node>>,
}

struct DoublyLinkedList {
    size: u32,
    head: Node,
    tail: Node,
}

impl DoublyLinkedList {
    fn new(&mut self) {
        self.size = 0;
        self.head = Node {
            key: -1,
            val: -1,
            next: None,
            prev: None,
        };
        self.tail = Node {
            key: -1,
            val: -1,
            next: None,
            prev: None,
        };
        if let node = self.head {
            node.next = Some(Box::new(self.tail));
        };
        // if let tail = self.tail {
        //     tail.prev = Some(Box::new(self.head));
        // }
        // DoublyLinkedList {
        //      size: 0,
        //      head: Node {
        //          key: -1,
        //          val: -1,
        //          next: None,
        //          prev: None,
        //      },
        //      tail: Node {
        //          key: -1,
        //          val: -1,
        //          next: None,
        //          prev: None,
        //      },
        //  }
    }
}
