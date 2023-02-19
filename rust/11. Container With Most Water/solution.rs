impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let mut max_area = 0i32;
        while left < right {
            let w = (right - left) as i32;
            let h = std::cmp::min(height[left], height[right]);
            max_area = std::cmp::max(max_area, w * h);
            if height[left] <= height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        max_area
    }
}
