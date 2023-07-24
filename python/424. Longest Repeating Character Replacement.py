from collections import defaultdict

class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        cntr = defaultdict(int)
        beg, end = 0, 0
        n = len(s)
        result = 1

        while end < n:
            cntr[s[end]] += 1
            cur_len = end - beg + 1
            while cur_len - max(cntr.values()) > k:
                cntr[s[beg]] -= 1
                beg += 1
                cur_len -= 1
            result = max(result, cur_len)
            end += 1
        return result   
