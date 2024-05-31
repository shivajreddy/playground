
s = "123 +1234 *"

curr = 0

curr_num = 0
while curr < len(s) - 1 and s[curr].isdigit():
    curr_num = (curr_num * 10) + int(s[curr])
    curr += 1

print(curr_num)


