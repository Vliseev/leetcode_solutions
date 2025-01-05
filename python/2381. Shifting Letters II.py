class Solution:
    def shiftingLetters(self, s: str, shifts: List[List[int]]) -> str:
        s = list(s)
        n = len(s)
        shift_arr = [0] * (n+1)

        for (b, e, d) in shifts:
            shift_arr[b] += 1 if d == 1 else -1
            shift_arr[e+1] -= 1 if d == 1 else -1

        for i in range(n):
            shift_arr[i+1] += shift_arr[i]
        for i in range(n):
            s[i] = chr((ord(s[i]) - ord('a') + shift_arr[i]) % 26 + ord('a'))
        return ''.join(s)
