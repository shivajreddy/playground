# print("\t\tPython Plaground Result")

from typing import List

class Solution:
    def findTargetSumWays(self, nums: List[int], target: int) -> int:

        total_sum = sum(nums)

        # if target is out of range, then 0 possibilities
        if abs(target) > total_sum: return 0

        # if (total_sum + target) is odd, then 0 possibilities
        if (total_sum + target) % 2 != 0: return 0

        # get the +ve sum we need
        postive_sum = (total_sum + target) // 2

        # initialize the DP table
        dp = [1] + [0] * (postive_sum)

        # fill the DP table
        for num in nums:
            for j in range(postive_sum, num - 1, -1):
                dp[j] += dp[j - num]

        return dp[-1]

s = Solution()
assert(1 == s.findTargetSumWays([1], 1))
assert(5 == s.findTargetSumWays([1, 1, 1, 1, 1], 3))
assert(4 == s.findTargetSumWays([0, 0, 1], 1))
assert(256 == s.findTargetSumWays([0, 0, 0, 0, 0, 0, 0, 0, 1], 1))

