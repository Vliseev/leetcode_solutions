impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::<Vec<i32>>::new();
        for i in 0..intervals.len() {
            if new_interval[1] < intervals[i][0] {
                result.push(new_interval);
                return [result, intervals[i..].to_vec()].concat()
            }
            else if intervals[i][1] < new_interval[0] {
                result.push(intervals[i].clone());
            }
            else {
                let left = std::cmp::min(intervals[i][0], new_interval[0]);
                let right = std::cmp::max(intervals[i][1], new_interval[1]);
                new_interval = vec![left, right];
            }
        }
        result.push(new_interval);
        result
    }
}
