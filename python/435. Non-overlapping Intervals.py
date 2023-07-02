def is_intersect(l1, r1, l2, r2):
    max_l = max(l1, l2)
    min_r = min(r1, r2)
    return max_l < min_r

class Solution:
    def eraseOverlapIntervals(self, intervals: List[List[int]]) -> int:
        result = 0
        intervals.sort(key = lambda x: x[0])
        prev = intervals[0]

        for cur in intervals[1:]:
            if is_intersect(*prev, *cur):
                if prev[1] > cur[1]:
                    prev = cur
                result += 1
            else:
                prev = cur
        return result
