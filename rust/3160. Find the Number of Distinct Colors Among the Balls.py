impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries.iter().fold(
            (std::collections::HashMap::new(), std::collections::HashMap::new(), vec![]),
            |(mut ball_dict, mut color_dict, mut result), q| {
                let ball = q[0];
                let color = q[1];
                if let Some(&color_ball) = ball_dict.get(&ball) {
                    if let Some(color_ball_count) = color_dict.get_mut(&color_ball) {
                        *color_ball_count -= 1;
                        if *color_ball_count == 0 {
                            color_dict.remove(&color_ball);
                        }
                    }
                }
                *color_dict.entry(color).or_insert(0) += 1;
                ball_dict.insert(ball, color);
                result.push(color_dict.len() as i32);
                (ball_dict, color_dict, result)
            },
        ).2
    }
}
