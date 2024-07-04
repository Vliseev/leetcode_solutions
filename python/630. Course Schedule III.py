from heapq import heappop, heappush
from typing import List

class Solution:
    def scheduleCourse(self, courses: List[List[int]]) -> int:
        pq = []
        total_time = 0
        courses = sorted(courses, key=lambda x: x[1])

        for dur, lastDay in courses:
            if total_time + dur <= lastDay:
                heappush(pq, -dur)
                total_time += dur
            elif pq and -pq[0] > dur:
                total_time += dur + heappop(pq)
                heappush(pq, -dur)
        return len(pq)
