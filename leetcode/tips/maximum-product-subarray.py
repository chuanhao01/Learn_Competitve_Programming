# https://leetcode.com/problems/maximum-product-subarray/description/
"""
Since we are finding max product in sub array, notice a few things
Since we are only dealing with integers, unless its 0, we always want to multiply it
Like the sum for Kadane's Algorithm, anytime we notice the product dropping to 0, we just reset it back taking the next non 0 number
Not sure how much i will remember this

TC: O(n) = O(2n)
Space: O(1)
"""

from typing import List

class Solution:
    def maxProduct(self, nums: List[int]) -> int:
        largest_so_far = nums[0]
        cur = 1
        for num in nums:
            cur *= num
            largest_so_far = max(largest_so_far, cur)
            if cur == 0:
                cur = 1
        cur = 1
        nums.reverse()
        for num in nums:
            cur *= num
            largest_so_far = max(largest_so_far, cur)
            if cur == 0:
                cur = 1
        return largest_so_far
