pub fn edit_distance(source: &str, target: &str) -> usize {
    let source: Vec<char> = source.chars().collect();
    let target: Vec<char> = target.chars().collect();

    let rows = source.len() + 1;
    let cols = target.len() + 1;

    let mut dp = vec![vec![0; cols]; rows];

    for i in 0..rows {
        dp[i][0] = i;
    }

    for j in 0..cols {
        dp[0][j] = j;
    }

    for i in 1..rows {
        for j in 1..cols {
            if source[i - 1] == target[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                let insert = dp[i][j - 1] + 1;
                let delete = dp[i - 1][j] + 1;
                let replace = dp[i - 1][j - 1] + 1;

                dp[i][j] = insert.min(delete).min(replace);
            }
        }
    }

    dp[source.len()][target.len()]
}