import heapq

input = [9, 7, 1, 3, 2, -1, 3, 5]
print(input)

arr = []

heapq.heapify(arr)


for num in input:
    heapq.heappush(arr, -num)


print(arr)

while arr:
    n = heapq.heappop(arr)
    print(-n)



