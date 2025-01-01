pub fn run<I>(input_data: I) -> i32
where
    I: Iterator<Item = String>,
{
    let mut result = 0;
    let keyword = "MAS";
    let window_height = keyword.len();
    let mut window: Vec<String> = Vec::new();
    let mut input_iter = input_data.peekable();

    while window.len() < window_height && input_iter.peek().is_some() {
        if let Some(line) = input_iter.next() {
            window.push(line);
        }
    }

    let first_char = keyword.chars().next().unwrap();
    let last_char = keyword.chars().rev().next().unwrap();

    while window.len() > 0 {
        if let Some(next_line) = input_iter.next() {
            window.push(next_line);
        }

        if let Some(line) = window.get(0) {
            for (column, char) in line.char_indices() {
                if char == first_char || char == last_char {
                    result += count_keyword_occurances(&window, column, keyword.to_string());
                    result +=
                        count_keyword_occurances(&window, column, keyword.chars().rev().collect());
                }
            }
        }

        window.remove(0);
    }

    result
}

fn find_word(
    window: &[String],
    start_col: usize,
    target: String,
    directions: (isize, isize),
) -> bool {
    for (i, target_ch) in target.chars().enumerate() {
        let new_row = (i as isize * directions.0) as usize;
        let new_col = (start_col as isize + i as isize * directions.1) as usize;

        if new_row >= window.len() || new_col >= window[new_row].len() {
            return false;
        }

        if window[new_row].chars().nth(new_col) != Some(target_ch) {
            return false;
        }
    }
    true
}

fn count_keyword_occurances(window: &[String], start_col: usize, target: String) -> i32 {
    let down_left = (1, -1);
    let down_right = (1, 1);
    let reversed_keyword = target.chars().rev().collect();

    if find_word(window, start_col, target.clone(), down_right)
        && (find_word(window, start_col + 2, target.clone(), down_left)
            || find_word(window, start_col + 2, reversed_keyword, down_left))
    {
        return 1;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input_data = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ];

        let result = run(input_data.into_iter());

        assert_eq!(result, 9);
    }
}
