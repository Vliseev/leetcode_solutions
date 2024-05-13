class Solution:
    def maxUniqueSplit(self, s: str) -> int:
        seen_words = set()
        max_words = 0

        def backtracking(beg, num, cur_s):
            nonlocal max_words
            if beg == len(s):
                max_words = max(max_words, num)
                return
            for end in range(beg + 1, len(s) + 1):
                cur_w = s[beg:end]
                if not cur_w in seen_words:
                    seen_words.add(cur_w)
                    backtracking(end, num + 1, cur_s + cur_w)
                    seen_words.remove(cur_w)

        backtracking(0, 0, "")
        return max_words
