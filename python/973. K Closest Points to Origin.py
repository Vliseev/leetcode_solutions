from heapq import heapify, heappop

class Solution:
    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:
        dist = [(x*x+y*y, [x, y]) for x, y in points]
        heapify(dist)

        result = []

        for _ in range(k):
            _, point = heappop(dist)
            result.append(point)
        return result

