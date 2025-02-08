struct NumberContainers {
    index2number: std::collections::HashMap<i32, i32>,
    number2heap: std::collections::HashMap<i32, std::collections::BinaryHeap<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {
    fn new() -> Self {
        NumberContainers {
            index2number: std::collections::HashMap::new(),
            number2heap: std::collections::HashMap::new(),
        }
    }
    
    fn change(&mut self, index: i32, number: i32) {
        self.index2number.insert(index, number);
        self.number2heap
            .entry(number)
            .or_insert_with(std::collections::BinaryHeap::new)
            .push(-index);
    }
    
    fn find(&mut self, number: i32) -> i32 {
        self.number2heap.get_mut(&number).and_then(|heap| {
            while let Some(&index) = heap.peek() {
                if self.index2number.get(&-index) == Some(&number) {
                    return Some(-index);
                }
                heap.pop();
            }
            None
        }).unwrap_or(-1)
    }
}

/**
 * Your NumberContainers object will be instantiated and called as such:
 * let obj = NumberContainers::new();
 * obj.change(index, number);
 * let ret_2: i32 = obj.find(number);
 */
