'''
get & put must run in O(1)

- must need an ordered list (at least: most significant item & least significant item)
get: for doing this in O(1),
    - we have to either store in a hashmap
    - tail node should be least priority item
put: for doing this in O(1)
    - cant iteratre to find a place to place the item in the collection

doubly-linked-list & hashmap of all node references.

tail <-> node <-> node <-> ... <-> node
          |
         pop



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
'''
