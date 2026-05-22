# https://leetcode.com/problems/3sum/description/
# Not really sure
"""
Sorting with 2 pointers
We want to fix 1 point, and check all possible 2 pointers
We then sort, because if we are too small we move the left pointer, if we are too big we move the right pointer

TC: O(n^2) = O(n log n) sort + O(n^2) 2 pointers I think
Space: O(n) for the set output

Not so sure about space
"""

from typing import List

class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        target = 0
        nums.sort()
        s = set()
        output = []
        for i in range(len(nums)):
            j = i + 1
            k = len(nums) - 1
            while j < k:
                sum = nums[i] + nums[j] + nums[k]
                if sum == target:
                    s.add((nums[i], nums[j], nums[k]))
                    j += 1
                    k -= 1
                elif sum < target:
                    j += 1
                else:
                    k -= 1
        output = list(s)
        return output
