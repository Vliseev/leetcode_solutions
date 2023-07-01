def is_intersect(l1, r1, l2, r2):
    max_l = max(l1, l2)
    min_r = min(r1, r2)
    return max_l <= min_r

class Solution:
    def merge(self, intervals: List[List[int]]) -> List[List[int]]:
        if len(intervals) <= 1:
            return intervals
        intervals.sort(key = lambda x: x[0])
        beg_int = intervals[0]
        result = []
        for cur_int in intervals[1:]:
            if is_intersect(*beg_int, *cur_int):
                beg_int[1] = max(cur_int[1], beg_int[1])
            else:
                result.append(beg_int)
                beg_int = cur_int
        result.append(beg_int)
        return result
