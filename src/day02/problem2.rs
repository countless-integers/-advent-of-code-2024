struct UnsafeLevelError {}

fn check_line(
    line: &str,
    safety_margin: i32,
    skip_index: Option<usize>,
) -> Result<(), UnsafeLevelError> {
    let numbers: Vec<i32> = line
        .split_whitespace()
        .enumerate()
        .filter_map(|(index, el)| {
            if Some(index) == skip_index {
                return None;
            }
            el.parse::<i32>().ok()
        })
        .collect();
    let mut previous: Option<i32> = None;
    let should_decrease: bool = numbers[0] > numbers[1];

    for &number in numbers.iter() {
        if previous.is_none() {
            previous = Some(number);
            continue;
        }

        let prev = previous.expect("This should not have happened");

        if number == prev || (number - prev).abs() > safety_margin {
            return Err(UnsafeLevelError {});
        }

        if number > prev && should_decrease == true {
            return Err(UnsafeLevelError {});
        } else if number < prev && should_decrease == false {
            return Err(UnsafeLevelError {});
        }

        previous = Some(number);
    }

    Ok(())
}

pub fn run<I>(input_data: I) -> i32
where
    I: Iterator<Item = String>,
{
    let mut safe_count = 0;
    let safety_margin = 3;

    for line in input_data {
        let line_result = check_line(&line, safety_margin, None);
        if line_result.is_ok() {
            safe_count += 1;
        } else {
            // brute force ftw 🤷
            for i in 0..line.len() {
                if check_line(&line, safety_margin, Some(i)).is_ok() {
                    safe_count += 1;
                    break;
                }
            }
        }
    }

    safe_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input_data = vec![
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ];

        let result = run(input_data.into_iter());

        assert_eq!(result, 4);
    }

    #[test]
    fn test_run_weird() {
        let input_data = vec!["1 6 4 2 1".to_string()];

        let result = run(input_data.into_iter());

        assert_eq!(result, 1);
    }
}
