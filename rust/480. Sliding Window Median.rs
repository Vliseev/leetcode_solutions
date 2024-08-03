use std::collections::{binary_heap::BinaryHeap, HashMap};

struct Solution;

struct MenianFinder {
    max_heap: BinaryHeap<i64>,
    min_heap: BinaryHeap<i64>,
    min_heap_size: usize,
    max_heap_size: usize,
    k: usize,
    remove_cache: HashMap<i64, usize>,
}

impl MenianFinder {
    fn new(k: usize) -> Self {
        MenianFinder {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
            min_heap_size: 0,
            max_heap_size: 0,
            k,
            remove_cache: HashMap::new(),
        }
    }

    fn get_median(&self) -> f64 {
        if self.k % 2 == 1 {
            *self.max_heap.peek().unwrap() as f64
        } else {
            let max = self.max_heap.peek().unwrap();
            let min = self.min_heap.peek().unwrap();
            (*max as f64 - *min as f64) / 2.0
        }
    }

    fn balance_heap(&mut self) {
        if self.min_heap_size > self.max_heap_size {
            self.max_heap.push(-self.min_heap.pop().unwrap());
            self.min_heap_size -= 1;
            self.max_heap_size += 1;
        }
    }

    fn add_num(&mut self, num: i32) {
        self.max_heap.push(num as i64);
        self.min_heap.push(-self.max_heap.pop().unwrap());
        self.min_heap_size += 1;
        self.balance_heap();
    }

    fn remove_num(&mut self, num: i32) {
        *self.remove_cache.entry(num as i64).or_insert(0) += 1;
        if num as i64 <= *self.max_heap.peek().unwrap() {
            self.max_heap_size -= 1;
        } else {
            self.min_heap_size -= 1;
        }
    }

    fn clean_heaps(&mut self) {
        while let Some(&top) = self.max_heap.peek() {
            if let Some(&count) = self.remove_cache.get(&top) {
                if count > 0 {
                    self.remove_cache.insert(top, count - 1);
                    self.max_heap.pop();
                    continue;
                }
            }
            break;
        }

        // Remove elements from min_heap
        while let Some(&top) = self.min_heap.peek() {
            if let Some(&count) = self.remove_cache.get(&-top) {
                if count > 0 {
                    self.remove_cache.insert(-top, count - 1);
                    self.min_heap.pop();
                    continue;
                }
            }
            break;
        }
    }
}

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        let mut mf = MenianFinder::new(k);
        let mut medians = Vec::new();

        if k > nums.len() {
            return medians;
        }

        for i in 0..k {
            mf.add_num(nums[i]);
        }

        medians.push(mf.get_median());

        for i in k..nums.len() {
            mf.remove_num(nums[i - k]);
            mf.add_num(nums[i]);
            mf.clean_heaps();
            medians.push(mf.get_median());
        }

        medians
    }
}

fn main() {
    let test_cases = vec![
        // Test case 1
        (
            vec![1, 3, -1, -3, 5, 3, 6, 7],
            3,
            vec![1.00000, -1.00000, -1.00000, 3.00000, 5.00000, 6.00000],
        ),
        // Test case 2
        (
            vec![1, 2, 3, 4, 2, 3, 1, 4, 2],
            3,
            vec![
                2.00000, 3.00000, 3.00000, 3.00000, 2.00000, 3.00000, 2.00000,
            ],
        ),
        // Test case 3: Asll elements are the same
        (vec![5, 5, 5, 5, 5], 3, vec![5.00000, 5.00000, 5.00000]),
        // Test case 4: Window size is 1
        (
            vec![1, 2, 3, 4, 5],
            1,
            vec![1.00000, 2.00000, 3.00000, 4.00000, 5.00000],
        ),
        // Test case 5: Window size equals the array size
        (vec![1, 2, 3, 4, 5], 5, vec![3.00000]), // Expected output: [3.00000]
        // Test case 6: Negative numbers
        (
            vec![-1, -2, -3, -4, -5],
            3,
            vec![-2.00000, -3.00000, -4.00000],
        ),
        // Test case 7: Mixed positive and negative numbers
        (
            vec![1, -1, 2, -2, 3, -3],
            3,
            vec![1.00000, -1.00000, 2.00000, -2.00000],
        ),
        // Test case 8: Large numbers
        (
            vec![1000000, 2000000, 3000000, 4000000, 5000000],
            3,
            vec![2000000.00000, 3000000.00000, 4000000.00000],
        ),
        // Test case 9: Small array with larger k
        (vec![1, 2], 3, vec![]), // Expected output: [] (no valid window)
        // Test case 10: Single element
        (vec![42], 1, vec![42.00000]),
        (
            vec![
                -2147483648,
                -2147483648,
                2147483647,
                -2147483648,
                -2147483648,
                -2147483648,
                2147483647,
                2147483647,
                2147483647,
                2147483647,
                -2147483648,
                2147483647,
                -2147483648,
            ],
            3,
            vec![
                -2147483648.00000,
                -2147483648.00000,
                -2147483648.00000,
                -2147483648.00000,
                -2147483648.00000,
                2147483647.00000,
                2147483647.00000,
                2147483647.00000,
                2147483647.00000,
                2147483647.00000,
                -2147483648.00000,
            ],
        ),
    ];

    for (nums, k, expected) in test_cases {
        let result = Solution::median_sliding_window(nums.clone(), k);
        assert!(
            (result.len() == expected.len())
                && result
                    .iter()
                    .zip(&expected)
                    .all(|(a, b)| (a - b).abs() < 1e-5),
            "Test failed for input: {:?}, k: {}. Expected: {:?}, Got: {:?}",
            nums,
            k,
            expected,
            result
        );
    }

    println!("All test cases passed!");
}
