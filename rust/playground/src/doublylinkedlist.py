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
