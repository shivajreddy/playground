/*
class Node:
    def __init__(self,
                key: int,
                val: int,
                next: 'Node | None' = None,
                prev: 'Node | None' = None
                ) -> None:
        self.key = key
        self.val = val
        self.next = next
        self.prev = prev


class DoublyLinkedList:
    def __init__(self):
        self.size = 0
        self.head = Node(-1, -1)
        self.tail = Node(-1, -1)
        self.head.next, self.tail.prev = self.tail, self.head   # head <-> tail

    def push(self, node: Node): # O(1)
        head, nxt = self.head, self.head.next   # head <-> nxt <-> ... <-> tail
        head.next, nxt.prev = node, node        # head  -> node <-  nxt     #type: ignore
        node.prev, node.next = head, nxt        # head <-> node <-> nxt
        self.size += 1

    def remove(self, node: Node) -> None: # O(1)
        # head <-> ... <-> node <-> ... <-> tail
        prev, nxt = node.prev, node.next        # prev <-> node <-> nxt
        node.prev, node.next = None, None       # prev  -> node <-  nxt
        prev.next, nxt.prev = nxt, prev         # prev <-> nxt   |  node    #type: ignore
        self.size -= 1

    def pop(self) -> Node: # O(1)
        node = self.tail.prev                   # head <-> ... <-> node <-> tail
        self.remove(node)       # type: ignore
        return node             # type: ignore
*/

fn main() {}

struct Node {
    key: i32,
    val: i32,
    next: Option<Box<Node>>,
    prev: Option<Box<Node>>,
}
impl Node {
    fn new(key: i32, val: i32, next: Option<Box<Node>>, prev: Option<Box<Node>>) -> Self {
        Node {
            key,
            val,
            next,
            prev,
        }
    }
}

struct DoublyLinkedList {
    size: u32,
    head: Box<Node>,
    tail: Box<Node>,
}

impl DoublyLinkedList {
    fn new() -> Self {
        let mut head = Box::new(Node::new(-1, -1));

        let mut dll = DoublyLinkedList {
            size: 0,
            head: Node::new(-1, -1, None, None),
            tail: Node::new(-1, -1, None, None),
        };
        dll.head.next = Some(Box::new(dll.tail));
        dll.tail.prev = Some(Box::new(dll.head));
        dll
    }
}

struct LRUCache {}

#[allow(unused)]
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
