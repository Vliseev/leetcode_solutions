class Solution:
    def longestWord(self, words: List[str]) -> str:
        words.sort()
        ws = set()
        is_cont = [False] * len(words)
        ans = ""

        for i, w in enumerate(words):
            if len(w) == 1:
                ws.add(w)
                if not ans:
                    ans = w
                ws.add(w)
            elif w[:-1] in ws:
                if len(w) > len(ans):
                    ans = w
                ws.add(w)
        return ans
