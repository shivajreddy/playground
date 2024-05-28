fn main() {}

struct LRUCache {}

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

