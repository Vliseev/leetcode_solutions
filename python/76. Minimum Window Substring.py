from collections import Counter


class Solution:
    def minWindow(self, s: str, t: str) -> str:
        cntr = Counter(t)
        window_chrtrs = {}
        left = 0
        required, formed = len(cntr), 0
        ans = float("inf"), None, None

        for right in range(len(s)):
            sym = s[right]
            window_chrtrs[sym] = window_chrtrs.get(sym, 0) + 1

            if sym in cntr and window_chrtrs[sym] == cntr[sym]:
                formed += 1

            while left <= right and formed == required:
                sym = s[left]
                if right - left + 1 < ans[0]:
                    ans = right - left + 1, left, right
                window_chrtrs[sym] -= 1
                if sym in cntr:
                    if window_chrtrs[sym] < cntr[sym]:
                        formed -= 1
                left += 1
        return "" if ans[0] == float("inf") else s[ans[1] : ans[2] + 1]


def test_minWindow():
    s = Solution()
    assert s.minWindow("ADOBECODEBANC", "ABC") == "BANC"
    assert s.minWindow("a", "a") == "a"
    assert s.minWindow("a", "aa") == ""
    assert s.minWindow("bba", "ab") == "ba"
    assert s.minWindow("bdabcae", "abc") == "abc"
    assert s.minWindow("abc", "b") == "b"
    assert s.minWindow("abc", "c") == "c"
    assert s.minWindow("abc", "d") == ""
    assert s.minWindow("aa", "aa") == "aa"
    assert s.minWindow("ab", "b") == "b"
    assert s.minWindow("ab", "a") == "a"
    assert s.minWindow("abc", "abc") == "abc"
    assert s.minWindow("abcc", "abc") == "abc"
    assert s.minWindow("abcb", "abc") == "abc"
    assert s.minWindow("abcbb", "abc") == "abc"
    print("All test cases pass.")


test_minWindow()
