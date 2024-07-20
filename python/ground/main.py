

sentence = "hellotherehowareyoudoingtoday"

# create frequency

# set

'''
- hashable items
{  val1 val2 val2 }

('a', 1)

'''

# list

counter = [0] * 26

# 'char' -> unicode, ASCII

# aabccc
# counter = [2 1 3 0 0 0 0 0 0 0 0 ]

sentence = "hellotherehowareyoudoingtoday"
for c in sentence:
    # 1. find the position of this character in the counter
    unicode_idx = ord(c)
    idx = unicode_idx - ord('a')
    counter[idx] += 1


# a: 0, b: 1
letter = 'a'
for count in counter:
    print(letter, ":", count)
    letter = chr(ord(letter) + 1)

    




