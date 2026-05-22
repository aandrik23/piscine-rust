pub fn solve_board(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len();

    if rows == 0 {
        return vec![];
    }

    let cols = minefield[0].len();

    let mut result = Vec::new();

    for row in 0..rows {
        let mut line = String::new();

        for col in 0..cols {
            if minefield[row].as_bytes()[col] == b'*' {
                line.push('*');
                continue;
            }

            let mut count = 0;

            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }

                    let nr = row as isize + dr;
                    let nc = col as isize + dc;

                    if nr >= 0
                        && nr < rows as isize
                        && nc >= 0
                        && nc < cols as isize
                        && minefield[nr as usize].as_bytes()[nc as usize] == b'*'
                    {
                        count += 1;
                    }
                }
            }

            if count == 0 {
                line.push(' ');
            } else {
                line.push(char::from_digit(count, 10).unwrap());
            }
        }

        result.push(line);
    }

    result
}