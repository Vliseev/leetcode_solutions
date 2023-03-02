impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::<Vec<i32>>::new();
        let mut cur_seq = Vec::<i32>::new();

        fn rec(
            idx: usize,
            candidates: &Vec<i32>,
            cur_sum: i32,
            target: i32,
            cur_seq: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if idx >= candidates.len() || cur_sum > target {
                return;
            }
            if cur_sum == target {
                result.push(cur_seq.clone());
                return;
            }

            cur_seq.push(candidates[idx]);
            rec(idx, candidates, cur_sum+candidates[idx], target, cur_seq, result);
            cur_seq.pop();
            rec(idx + 1, candidates, cur_sum, target, cur_seq, result);
        }
        rec(0, &candidates, 0, target, &mut cur_seq, &mut result);
        result
    }
}

