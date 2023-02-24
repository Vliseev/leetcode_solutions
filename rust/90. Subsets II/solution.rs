pub struct Rec<'a> {
    result: Vec<Vec<i32>>,
    cur: Vec<i32>,
    nums: &'a Vec<i32>,
}

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut rec_ctx = Rec {
            result: Vec::<Vec<i32>>::new(),
            cur: Vec::<i32>::new(),
            nums: &nums,
        };
        fn rec(mut i: usize, rec_ctx: &mut Rec) {
            let nums = rec_ctx.nums;

            if i == nums.len() {
                rec_ctx.result.push(rec_ctx.cur.clone());
                return;
            }
            rec_ctx.cur.push(nums[i]);
            rec(i + 1, rec_ctx);
            rec_ctx.cur.pop();
            while i < nums.len() - 1 && nums[i] == nums[i + 1] {
                i += 1;
            }
            rec(i + 1, rec_ctx);
        }
        rec(0, &mut rec_ctx);
        rec_ctx.result
    }
}

