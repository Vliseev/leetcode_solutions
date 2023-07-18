impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, mut k: i32, x: i32) -> Vec<i32> {
        let mut l = 0i32;
        let mut r = arr.len() as i32;
        while l < r {
            let m = (l + r) / 2;
            if arr[m as usize] < x {
                l = m + 1;
            } else {
                r = m;
            }
        }
        r = l;
        l = r - 1;
        while k > 0 {
            if l < 0 {
                r += 1;
            } else if r == arr.len() as i32 {
                l -= 1;
            } else if (arr[l as usize] - x).abs() <= (arr[r as usize] - x).abs() {
                l -= 1;
            } else {
                r += 1;
            }
            k -= 1;
        }

        arr[(l + 1) as usize..r as usize].to_vec()
    }
}
