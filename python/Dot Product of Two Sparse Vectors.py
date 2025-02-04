from typing import (
    List,
)

class SparseVector:
    # Your SparseVector object will be instantiated and called as such:
    # v1 = SparseVector(nums1)
    # v2 = SparseVector(nums2)
    # ans = v1.dot_product(v2)
    def __init__(self, nums: List[int]):
        # do intialization here
        self.nonzero = {}
        for i, num in enumerate(nums):
            if num != 0:
                self.nonzero[i] = num

    # Return the dotProduct of two sparse vectors
    def dot_product(self, vec: "SparseVector") -> int:
        smallest = self if len(self.nonzero) < len(vec.nonzero) else vec
        largest = self if len(self.nonzero) >= len(vec.nonzero) else vec
        result = 0
        for i, num in smallest.nonzero.items():
            if i in largest.nonzero:
                result += num * largest.nonzero[i]
        return result
