impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut l = 0usize;
        let mut r = arr.len() - 1;
        while l < r{
            let m = l + (r-l) / 2;
            if arr[m] < arr[m+1] {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l as i32
    }
}
