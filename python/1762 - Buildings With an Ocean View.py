from typing import (
    List,
)

class Solution:
    """
    @param heights: An integer array
    @return: Indexs of buildings with sea view
    """
    def find_buildings(self, heights: List[int]) -> List[int]:
        # write your code here
        n = len(heights)
        res = [n-1]
        max_height = heights[-1]

        for i in range(n - 2, -1, -1):
            if heights[i] > max_height:
                res.append(i)
                max_height = heights[i]
            
        return res[::-1]
