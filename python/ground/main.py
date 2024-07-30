# print("\t\tPython Plaground Result")

'''
Delete Operation for Two Strings

min no.of steps?
Goal: word1 == word2


word1 = sea
word2 = eat

word1 = leetcode
word2 = etco

l -> 0
e -> 1
e -> 

abcde
aababcabcdabcde

early stop if common == size_1 or size_2
a -> [ 1 2 3 4 5] -> 5 

abcde
abadbcde

a -> [1 2] -> 2
b -> [1 4] -> 4

abadbcde
abcde

a -> [1 2]
b -> [1]
a -> [1]
d -> [1]
b -> [1 2 3 4] -> early_stop

   i   j
   aaaaab
   |___| -1
    0123456789012345678
    aaaaaaaaaaaaaaaaaab
    |___| (0-4)
     |___| (1-4)(5) (1-5)
      |___| (2-5)(6) (2-6)
      |___| (2-5)(6) (2-6)
      ...
                  |___| (13-17)(18) (13-18)

word1 = a
word2 = a

word1 = a
word2 = b

Observations:
for 2 strings to be equal: same size, frequency, prefix, suffix
- finding the largest common pattern i.e., common
- min. steps = (size_1 - common) + (size_2 - common)


'''


"""
class Solution:
    def minDistance(self, word1: str, word2: str) -> int:
        cache = {}
        def helper(i, j):
            if i >= len(word1): return len(word2) - j
            if j >= len(word2): return len(word1) - i
            if (i, j) not in cache:
                if word1[i] == word2[j]:
                    cache[(i, j)] = helper(i+1, j+1)
                else:
                    cache[(i, j)] = 1 + min(
                        helper(i+1, j), helper(i, j+1)
                    )

            return cache[(i, j)]
        return helper(0, 0)


s = Solution()
assert(2 == s.minDistance("sea", "eat"))
# """

class Solution:
    def longestCommonSubSequence(self, word1: str, word2: str) -> int:

        cache = {}

        def helper(i: int, j: int) -> int:
            if i >= len(word1) or j >= len(word2):
                return 0
            if (i,j) not in cache:
                if word1[i] == word2[j]:
                    val = 1 + helper(i+1, j+1)
                else:
                    val = max(
                        helper(i+1, j),
                        helper(i, j+1)
                    )
                cache[(i, j)] = val
            return cache[(i, j)]

        return helper(0, 0)

s = Solution()

assert(2 == s.longestCommonSubSequence("bd", "abcd"))
assert(2 == s.longestCommonSubSequence("oxcpqrsvwf", "shmtulqrypy"))


