class Solution:
    def frequencySort(self, s: str) -> str:
        freq_ch = [[i, 0] for i in range(123)]
        for el in s:
            freq_ch[ord(el)][1] += 1
        freq_ch.sort(key=lambda x: -x[1])
        result = []
        for sym_id, cnt in freq_ch:
            if cnt > 0:
                sym = chr(sym_id)
                result.append(sym*cnt)
            else:
                break
        return "".join(result)
