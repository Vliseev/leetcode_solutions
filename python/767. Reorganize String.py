from collections import Counter
from heapq import heapify, heappop, heappush

class Solution:
    def reorganizeString(self, s: str) -> str:
        cntr = Counter(s)
        hp = [(-cnt, c) for c, cnt in cntr.items()]
        heapify(hp)
        res = ""
        prev = None
        while hp or prev:
            if prev and not hp:
                return ""
            cnt, c = heappop(hp)
            res += c
            if prev:
                heappush(hp, prev)
            cnt += 1
            if cnt != 0:
                prev = (cnt, c)
            else:
                prev = None
        return res

