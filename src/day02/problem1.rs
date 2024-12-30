pub fn run<I>(input_data: I) -> i32
where
    I: Iterator<Item = String>,
{
    let mut safe_count = 0;
    let safety_margin = 3;

    for line in input_data {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|el| el.parse::<i32>().ok())
            .collect();
        let mut is_safe = false;
        let mut previous: Option<i32> = None;
        let mut is_decreasing: Option<bool> = None;

        for &number in &numbers {
            if previous.is_none() {
                previous = Some(number);
                continue;
            }

            let prev = previous.unwrap();

            if (number - prev).abs() > safety_margin || number == prev {
                is_safe = false;
                break;
            }

            if number > prev {
                if let Some(true) = is_decreasing {
                    is_safe = false;
                    break;
                }
                is_safe = true;
                is_decreasing = Some(false);
                previous = Some(number);
                continue;
            }

            if number < prev {
                if let Some(false) = is_decreasing {
                    is_safe = false;
                    break;
                }
                is_safe = true;
                is_decreasing = Some(true);
                previous = Some(number);
                continue;
            }
        }

        if is_safe {
            safe_count += 1;
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

        assert_eq!(result, 2);
    }
}
