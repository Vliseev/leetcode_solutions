pub struct Solution;
// Definition of Interval:
// #[derive(Debug, PartialEq, Eq)]
// pub struct Interval {
//     pub start: i32,
//     pub end: i32
// }
//
// impl Interval {
//     #[inline]
//     pub fn new(start: i32, end: i32) -> Self {
//         Interval {start: start, end: end}
//     }
// }

enum Type {
    Start,
    End,
}

impl Solution {
    // @param intervals: an array of meeting time intervals
    // @return: the minimum number of conference rooms required
    pub fn min_meeting_rooms(intervals: Vec<Interval>) -> i32 {
        let mut all_bound = Vec::<(i32, Type)>::new();
        for interv in &intervals {
            all_bound.push((interv.start, Type::Start));
            all_bound.push((interv.end, Type::End));
        }
        all_bound.sort_by_key(|k| k.0);

        let mut cur_val = 0i32;
        let mut result = 1i32;

        for (_, v_type) in &all_bound {
            match v_type {
                Type::Start => cur_val += 1,
                Type::End => cur_val -= 1,
            }
            result = std::cmp::max(cur_val, result);
        }

        result
    }
}

/*
#[macro_export]
macro_rules! intervals {
    () => {
        vec![]
    };
    ($($e:expr), *) => {
        {
            let mut res = vec![];
            $(
                res.push(Interval{start:$e.0, end:$e.1});
            )*
            res
        }
    }
}
let inter = intervals![(0, 30), (5, 10), (15, 20)];
 */
