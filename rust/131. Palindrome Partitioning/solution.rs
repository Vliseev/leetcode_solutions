fn is_poly(mut i: usize, mut j: usize, s: &str) -> bool {
    let chars = s.chars().collect::<Vec<_>>();
    while i < j {
        if chars[i] != chars[j] {
            return false;
        }
        i+=1;
        j-=1;
    }
    true
}

fn rec(i: usize, s: &String, cur: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
    if i >= s.len() {
        res.push(cur.clone());
    }

    let n = s.len();
    for j in i..n {
        if is_poly(i, j, s) {
            cur.push(s[i..j + 1].to_string());
            rec(j + 1, s, cur, res);
            cur.pop();
        }
    }
}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res = Vec::<Vec<String>>::new();
        let mut cur = Vec::<String>::new();
        rec(0, &s, &mut cur, &mut res);
        res
    }
}
