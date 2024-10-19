class Solution:
    def findKthBit(self, n: int, k: int) -> str:
        def rec_find(n: int, k: int) -> int:
            if n == 1:
                return 0
            prev_length = (1 << (n - 1))

            if k == prev_length:
                return 1

            if k < prev_length: # **Sn-1** || 1 || rev(inv(Sn-1))
                return rec_find(n - 1, k)

            return 1 - rec_find(n - 1, prev_length * 2 - k) # Sn-1 || 1 || **rev(inv(Sn-1))**
        return str(rec_find(n, k))
