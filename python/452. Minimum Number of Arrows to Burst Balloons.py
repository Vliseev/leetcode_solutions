class Solution:
    def findMinArrowShots(self, points: List[List[int]]) -> int:
        num = 1
        points.sort(key = lambda x: x[1])

        prev = points[0]
        for cur in points[1:]:
            if cur[0] > prev[1]:
                prev = cur
                num += 1
        return num
