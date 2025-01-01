pub fn run<I>(input_data: I) -> i32
where
    I: Iterator<Item = String>,
{
    let mut result = 0;
    let keyword = "XMAS";
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
                    result += count_keyword_occurances(&window, column, keyword.chars().rev().collect());
                }
            }
        }

        window.remove(0);
    }

    result
}

fn count_keyword_occurances(window: &[String], start_col: usize, target: String) -> i32 {
    let directions = [
        (1, 0, "down"),
        (0, 1, "right"),
        (1, -1, "down-left"),
        (1, 1, "down-right"),
    ];

    let mut found_times = 0;
    for &(dx, dy, _dir) in &directions {
        let mut found = true;

        for (i, target_ch) in target.chars().enumerate() {
            let new_row = (i as isize * dx) as usize;
            let new_col = (start_col as isize + i as isize * dy) as usize;

            if new_row >= window.len() || new_col >= window[new_row].len() {
                found = false;
                break;
            }

            if window[new_row].chars().nth(new_col) != Some(target_ch) {
                found = false;
                break;
            }
        }

        if found {
            found_times += 1
        }
    }

    found_times
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

        assert_eq!(result, 18);
    }
}
