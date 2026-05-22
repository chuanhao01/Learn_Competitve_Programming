# https://leetcode.com/problems/rotate-image/description/
"""
Funny Math and rotations
Also ordering is kinda funny

TC: O(n^2) since we "touch" all fields in the matrix
Space: O(1)
"""

from typing import List

class Solution:
    def rotate(self, matrix: List[List[int]]) -> None:
        """
        Do not return anything, modify matrix in-place instead.
        """
        def rotate(i, n):
            # Base Case
            if n - i < 2:
                return
            tl, tr, br, bl = (i, i), (i, n - 1),  (n - 1, n - 1), (n - 1, i)
            for _ in range(i, n - 1):
                # print(tl, tr, br, bl)
                tmp = matrix[bl[0]][bl[1]]
                matrix[bl[0]][bl[1]] = matrix[br[0]][br[1]]
                matrix[br[0]][br[1]] = matrix[tr[0]][tr[1]]
                matrix[tr[0]][tr[1]] = matrix[tl[0]][tl[1]]
                matrix[tl[0]][tl[1]] = tmp

                tl = (tl[0], tl[1] + 1)
                tr = (tr[0] + 1, tr[1])
                br = (br[0], br[1] - 1)
                bl = (bl[0] - 1, bl[1])
            rotate(i + 1, n - 1)
        rotate(0, len(matrix))
