from collections import deque

d = deque([])


nums = [1,3,3,-1,-3,5,3,6,7]
k = 3

result = []

l, r = 0, 0
while r < len(nums):

    new_item = nums[r]

    if not d or new_item > d[0]:
        d = deque([new_item])
    else:
        while d and d[-1] < new_item:
            d.pop()
        d.append(new_item)

    if r - l + 1 == k:
        result.append(d[0])
        if nums[l] == d[0]:
            d.popleft()
        l += 1

    r += 1


print(result)
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


