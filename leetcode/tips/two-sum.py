# https://leetcode.com/problems/two-sum/
"""
Use a counts hashmap to keep track of compliments
Since we always have a solution, return the first solution

TC: O(n)
Space: O(n) for comps hm
"""

from typing import List

class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        comps = {}
        for i in range(len(nums)):
            num = nums[i]
            c = target - num
            if c in comps:
                return [i, comps[c]]
            else:
                comps[num] = i



if __name__ == "__main__":
    sol = Solution()
