# https://leetcode.com/problems/contains-duplicate/
"""
Create a set to track previously seen nums
If we encounter a num we have seen before terminate

TC: O(n)
Space: O(n)
"""

from typing import List

class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        prev = set()
        for num in nums:
            if num in prev:
                return True
            else:
                prev.add(num)
        return False
