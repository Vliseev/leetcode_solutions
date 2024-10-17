impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut nums: Vec<_> = num.to_string().chars().collect();
        let mut last_id: Vec<isize> = vec![-1; 10];
        for (i, &digit) in nums.iter().enumerate() {
            last_id[digit.to_digit(10).unwrap() as usize] = i as isize;
        }
        for (i, &digit) in nums.iter().enumerate() {
            let d = digit.to_digit(10).unwrap();
            for ld in (d+1..10).rev() {
                if last_id[ld as usize] > i as isize {
                    nums.swap(i, last_id[ld as usize] as usize);
                    return nums.iter().collect::<String>().parse::<i32>().unwrap();
                }
            }
        }
        num
    }
}
