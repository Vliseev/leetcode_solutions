class NumberContainers:

    def __init__(self):
        self.index2number = {}
        self.number2heap = {}

    def change(self, index: int, number: int) -> None:
        self.index2number[index] = number
        if number in self.number2heap:
            heapq.heappush(self.number2heap[number], index)
        else:
            self.number2heap[number] = [index]
        

    def find(self, number: int) -> int:
        if number not in self.number2heap:
            return -1
        else:
            heap = self.number2heap[number]
            while heap:
                index = heap[0]
                if self.index2number.get(index, -1) == number:
                    return index
                else:
                    heapq.heappop(heap)
            return -1
        


# Your NumberContainers object will be instantiated and called as such:
# obj = NumberContainers()
# obj.change(index,number)
# param_2 = obj.find(number)
