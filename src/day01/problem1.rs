pub fn run<I>(input_data: I) -> i32
where
    I: Iterator<Item = String>,
{
    let mut col1 = vec![];
    let mut col2 = vec![];

    for line in input_data {
        let columns: Vec<&str> = line.split_whitespace().collect();

        if let (Ok(num1), Ok(num2)) = (columns[0].parse::<i32>(), columns[1].parse::<i32>()) {
            col1.push(num1);
            col2.push(num2);
        } else {
            eprintln!("Failed to parse numbers on line: {}", line);
        }
    }

    col1.sort();
    col2.sort();
    let mut total_distance = 0;
    for (index, value) in col1.iter().enumerate() {
        let distance = (value - col2[index]).abs();
        total_distance += distance;
    }

    return total_distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input_data = vec![
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string(),
        ];

        let result = run(input_data.into_iter());

        assert_eq!(result, 11);
    }
}
