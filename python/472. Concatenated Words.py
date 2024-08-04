class Solution:
    def findAllConcatenatedWordsInADict(self, words: List[str]) -> List[str]:
        word_set = set(words)
        memo = dict()
        words.sort()

        def dfs(word):
            if word in memo:
                return memo[word]
            for i in range(1, len(word)):
                pref = word[:i]
                suf = word[i:]
                if pref in word_set and (suf in word_set or dfs(suf)):
                    memo[word] = True
                    return True
            memo[word] = False
            return False

        result = []
        for w in words:
            if dfs(w):
                result.append(w)
        return result
