from collections import defaultdict

class FreqStack:
    def __init__(self):
        self.freq_count = defaultdict(int)
        self.max_freq = 0
        self.freq_dict = defaultdict(list)

    def push(self, val: int) -> None:
        freq = self.freq_count[val] = self.freq_count[val] + 1
        if freq > self.max_freq:
            self.max_freq = freq
        self.freq_dict[freq].append(val)

    def pop(self) -> int:
        val = self.freq_dict[self.max_freq].pop()
        self.freq_count[val] -= 1
        if not self.freq_dict[self.max_freq]:
            self.max_freq -= 1
        return val
        


# Your FreqStack object will be instantiated and called as such:
# obj = FreqStack()
# obj.push(val)
# param_2 = obj.pop()
