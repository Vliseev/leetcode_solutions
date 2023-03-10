impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::<String>::new();
        let mut cur = Vec::<char>::new();
        fn rec_foo(
            opened: i32,
            closed: i32,
            n: i32,
            cur: &mut Vec<char>,
            result: &mut Vec<String>,
        ) {
            if opened < n {
                cur.push('(');
                rec_foo(opened + 1, closed, n, cur, result);
                cur.pop();
            }
            if closed < opened {
                cur.push(')');
                rec_foo(opened, closed + 1, n, cur, result);
                cur.pop();
            }
            if opened == closed && opened == n {
                let val = cur.iter().collect::<String>();
                result.push(val);
            }
        }
        rec_foo(0, 0, n, &mut cur, &mut result);
        result
    }
}

