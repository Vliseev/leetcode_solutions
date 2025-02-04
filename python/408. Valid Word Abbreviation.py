class Solution:
    """
    @param word: a non-empty string
    @param abbr: an abbreviation
    @return: true if string matches with the given abbr or false
    """
    def valid_word_abbreviation(self, word: str, abbr: str) -> bool:
        # write your code here
        i, j = 0, 0
        n, m = len(word), len(abbr)
        while i < n and j < m:
            if abbr[j].isdigit():
                if abbr[j] == '0':
                    return False
                num = 0
                while j < m and abbr[j].isdigit():
                    num = num * 10 + int(abbr[j])
                    j += 1
                i += num
            else:
                if word[i] != abbr[j]:
                    return False
                i += 1
                j += 1
        return i == n and j == m

