use std::collections::HashSet;

pub fn run<I>(input_data: I) -> i64
where
    I: Iterator<Item = String>,
{
    let mut total: i64 = 0;
    for line in input_data {
        let (target, numbers) = line.split_once(':').expect("Expected : to split on");
        let target: i64 = target.parse().expect("Expected target to be an int");
        let numbers: Vec<i64> = numbers
            .trim()
            .split(' ')
            .map(|s| s.parse().expect("Expected numbers to be ints"))
            .collect();

        let mut results: Vec<HashSet<i64>> = vec![HashSet::from([numbers[0]])];
        for i in 1..numbers.len() {
            let number = numbers[i];
            let mut new_results = HashSet::new();
            
            for prev_num in &results[i-1] {
                let added: i64 = prev_num + number;
                let mult: i64 = prev_num * number;

                if added <= target {
                    new_results.insert(added);
                }
                if mult <= target {
                    new_results.insert(mult);
                }
            }
            results.push(new_results);
        }

        for res in results.last().unwrap() {
            if *res == target {
                total += res;
                break;
            }
        }
    }
    
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input_data = vec![
            "190: 10 19".to_string(),
            "3267: 81 40 27".to_string(),
            "83: 17 5".to_string(),
            "156: 15 6".to_string(),
            "7290: 6 8 6 15".to_string(),
            "161011: 16 10 13".to_string(),
            "192: 17 8 14".to_string(),
            "21037: 9 7 18 13".to_string(),
            "292: 11 6 16 20".to_string(),
        ];

        let result = run(input_data.into_iter());

        assert_eq!(result, 3749);
    }
}
