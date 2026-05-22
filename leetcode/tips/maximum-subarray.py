# https://leetcode.com/problems/maximum-subarray/description/
"""
Kadane's Algorithm
Going for left to right, if taking the new num makes it more than 0, we set the max larger
If we go below 0, we just reset from the next number

TC: O(n)
Space: O(1) since its only 3 variables
"""

from typing import List

class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        largest_so_far = nums[0]
        total = 0
        for num in nums:
            total += num
            largest_so_far = max(largest_so_far, total)
            if total < 0:
                total = 0
        return largest_so_far
