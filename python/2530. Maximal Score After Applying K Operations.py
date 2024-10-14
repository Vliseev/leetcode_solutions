from typing import List
import heapq
import math

class Solution:
    def maxKelements(self, nums: List[int], k: int) -> int:
        score = 0
        elems = [-el for el in nums]
        heapq.heapify(elems)

        for _ in range(k):
            el = -heapq.heappop(elems)
            score += el
            new_val = math.ceil(el/3)
            heapq.heappush(elems, -new_val)
        return score
