'''
Design LRU

# 1. Create a Node

'''

"""
Node1 <-> Node2 <-> Node3 -> None
A node can atmost have 1 node as its next
A node can atmost have 1 node as its prev
This ensures there is no branching or circular linked lists
"""
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

"""

head <-> node1 <-> node2 <-> ... <-> nodeN <-> tail

"""
from typing import Dict

class DoublyLinkedList:
    def __init__(self) -> None:
        self.head = Node(-1, -1)
        self.tail= Node(-1, -1)
        self.head.next, self.tail.prev = self.tail, self.head   # head <-> tail

    # remove a given node
    def remove(self, target: Node) -> None:
        prev, nxt = target.prev, target.next    # prev <-> target <-> nxt
        target.prev, target.next = None, None   # prev  -> target <-  nxt
        prev.next, nxt.prev = nxt, prev         # prev <-> nxt  |  target

    # remove the node previous to tail
    def pop(self) -> None:
        target = self.tail.prev     # head <-> node <-> ... <-> target <-> prev
        self.remove(target)

    # add new node next to head
    def push(self, node: Node):
        head, nxt = self.head, self.head.next   # head <-> nxt <-> ... <-> prev
        head.next, nxt.prev = node, node        # head  -> new_node <-  nxt
        node.prev, node.next = head, nxt        # head <-> new_node <-> nxt


class LRUCache:
    def __init__(self, capacity: int) -> None:
        self.capacity = capacity
        self.cache = DoublyLinkedList()
        self.mapper: Dict[int, int] = {}

    def get(self, key: int) -> int:
        if key not in self.mapper:
            return -1
        return self.mapper[key]

    def put(self, key: int, value: int) -> None:
        pass


