from collections import deque

d = deque([])

d.appendleft(10)
d.append(20)
d.appendleft(30)

print(d)

#  Monotonic Queue
'''
    1  3  3  -1  -3  5  3  6  7
    -
    1

    1  3  3  -1  -3  5  3  6  7
    ----
    3

    1  3  3  -1  -3  5  3  6  7
    -------
    3,3

    1  3  3  -1  -3  5  3  6  7
       --------
    3,3,-1  drop item not >= head

    1  3  3  -1  -3  5  3  6  7
          ---------
    3,-1,-3  drop item >= head

    1  3  3  -1  -3  5  3  6  7
             ---------
    5 new_item > head

    1  3  3  -1  -3  5  3  6  7
                 --------
    5,3 drop_item not >= head

    1  3  3  -1  -3  5  3  6  7
                     -------
    6 new_item > head

    1  3  3  -1  -3  5  3  6  7
                        -------
    7 new_item > head


    when shifting:
        - if dropped item is highest item, pop head
        - if new item strictly > highest, window holds only this item
            else: pop off every item from back until new_item <= last item


'''


