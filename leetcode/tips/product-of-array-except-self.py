# https://leetcode.com/problems/product-of-array-except-self/description/
"""
We build a prefix and suffix product array
Using this, for any position, we multiply the prefix before and suffix after to get product excluding it

TC: O(n) = O(3n)
Space: O(1)? Since we can make the ans array and have the original array
"""

from typing import List

class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        n = len(nums)
        prefix_prod = []
        prefix = 1
        for num in nums:
            prefix *= num
            prefix_prod.append(prefix)
        suffix_prod = [0 for n in range(n)]
        suffix = 1
        for i in range(n-1, -1, -1):
            suffix *= nums[i]
            suffix_prod[i] = suffix
        ans = []
        for i in range(n):
            cur_ans = 1
            if i > 0:
                cur_ans *= prefix_prod[i-1]
            if i < n-1:
                cur_ans *= suffix_prod[i+1]
            ans.append(cur_ans)
        return ans
