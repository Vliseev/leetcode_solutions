impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        fn rec_foo(n: i32, k: i32) -> i32 {
            if n == 1 {
                return 0;
            }
            let prev_length = (1i32 << (n - 1));
            if k == prev_length {
                return 1;
            }

            if k < prev_length {
                return rec_foo(n - 1, k);
            }
            return 1 - rec_foo(n - 1, 2 * prev_length - k);
        }
        match rec_foo(n, k) {
            1 => '1',
            _ => '0',
        }
    }
}
