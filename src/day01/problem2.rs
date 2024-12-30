use std::collections::HashMap;

pub fn run<I>(input_data: I) -> i32
where
    I: Iterator<Item = String>,
{
    let mut col1: Vec<i32> = Vec::new();
    let mut counts = HashMap::new();

    for line in input_data {
        let columns: Vec<&str> = line.split_whitespace().collect();

        if let (Ok(num1), Ok(num2)) = (columns[0].parse::<i32>(), columns[1].parse::<i32>()) {
            col1.push(num1);
            *counts.entry(num2).or_insert(0) += 1;
        } else {
            println!("Failed to parse numbers on line: {}", line);
        }
    }

    let mut similarity_score = 0;
    for value in col1 {
        let similarity = value * *counts.get(&value).unwrap_or(&0);
        similarity_score += similarity;
    }

    return similarity_score
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

        assert_eq!(result, 31);
    }
}
