class Solution:
    def findClosestElements(self, arr: List[int], k, x) -> List[int]:
        l = 0 
        r = len(arr) - 1
        while l < r:
            mid = (l + r) // 2
            if arr[mid] < x:
                l = mid + 1
            else: r = mid
        r = l
        l = r - 1
        while k > 0:
            if l < 0:
                r += 1
            elif r == len(arr):
                l -= 1
            elif abs(arr[l]-x) <= abs(arr[r]-x):
                l -= 1
            else:
                r += 1
            k -= 1
        return arr[l+1:r]
