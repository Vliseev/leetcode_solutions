from heapq import heappop, heappush, heapify

class Solution:
    def kSmallestPairs(self, nums1: List[int], nums2: List[int], k: int) -> List[List[int]]:
        hp = []
        result = []

        visited = set((0, 0))
        heappush(hp, (nums1[0]+nums2[0], 0, 0))

        def update(idx1, idx2):
            if not (idx1, idx2) in visited:
                visited.add((idx1, idx2))
                heappush(hp, (nums1[idx1]+nums2[idx2], idx1, idx2))

        while k and hp:
            _, idx1, idx2 = heappop(hp)
            if idx1 + 1 < len(nums1):
                update(idx1 + 1, idx2)
            if idx2 + 1 < len(nums2):
                update(idx1, idx2 + 1)
            k -= 1
            result.append([nums1[idx1], nums2[idx2]])

        return result
