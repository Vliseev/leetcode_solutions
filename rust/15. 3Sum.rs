impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut result = Vec::new();

        let n = nums.len();

        for i in 0..n - 2 {
            if i > 0 && nums[i - 1] == nums[i] {
                continue;
            }
            let (mut j, mut k) = (i + 1, n - 1);
            while j < k {
                let cur_sum = nums[i] + nums[j] + nums[k];
                match cur_sum.cmp(&0) {
                    std::cmp::Ordering::Equal => {
                        result.push(vec![nums[i], nums[j], nums[k]]);

                        while j < k && nums[j] == nums[j + 1] {
                            j += 1;
                        }
                        while j < k && nums[k] == nums[k - 1] {
                            k -= 1;
                        }

                        j += 1;
                        k -= 1;
                    }
                    std::cmp::Ordering::Less => j += 1,
                    std::cmp::Ordering::Greater => k -= 1,
                }
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![vec![-1, -1, 2], vec![-1, 0, 1]]
    );
    assert_eq!(Solution::three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
    assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
}
