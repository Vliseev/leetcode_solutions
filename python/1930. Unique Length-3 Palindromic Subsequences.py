class Solution:
    def countPalindromicSubsequence(self, s: str) -> int:
        first_occ = {}
        last_occ = {}
        for i, ch in enumerate(s):
            if not ch in first_occ:
                first_occ[ch] = i
            last_occ[ch] = i
        result = 0

        for ch in first_occ:
            first = first_occ[ch]
            last = last_occ[ch]
            if first < last:
                uniq_sym = set(s[first+1:last])
                result += len(uniq_sym)
        return result
