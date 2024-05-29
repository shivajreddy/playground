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
class DoublyLinkedList:
    def __init__(self) -> None:
        self.head = Node(-1, -1)
        self.tail= Node(-1, -1)


class LRUCache:
    def __init__(self, capacity: int) -> None:
        self.capacity = capacity

