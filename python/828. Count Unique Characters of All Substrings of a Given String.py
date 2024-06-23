class Solution:
    """
    sum(countUniqueChars(t)) = sum(t(c - is unique))
    """
    def uniqueLetterString(self, s: str) -> int:
        n = len(s)
        prev = [-1] * n
        next = [n] * n
        
        pos = {}
        for i in range(n):
            ch = s[i]
            if ch in pos:
                prev[i] = pos[ch]
            pos[ch] = i
        
        pos = {}
        for i in range(n - 1, -1, -1):
            ch = s[i]
            if ch in pos:
                next[i] = pos[ch]
            pos[ch] = i
        
        result = 0
        for i in range(n):
            left_cnt = i - prev[i]
            right_cnt = next[i] - i
            result += left_cnt * right_cnt
        
        return result
