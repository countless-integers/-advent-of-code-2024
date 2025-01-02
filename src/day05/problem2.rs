use std::collections::HashMap;

pub fn run<I>(input_data: I) -> i32
where
    I: Iterator<Item = String>,
{
    let mut validation_rules: HashMap<i32, HashMap<&str, Vec<i32>>> = HashMap::new();
    let mut result = 0;

    let mut data = input_data.into_iter();
    for line in data.by_ref() {
        if line.trim().is_empty() {
            break;
        }
        let (first, second) = parse_validation_rule(line);

        validation_rules
            .entry(first)
            .or_insert_with(|| HashMap::from([("before", vec![]), ("after", vec![])]))
            .get_mut("before")
            .unwrap()
            .push(second);

        validation_rules
            .entry(second)
            .or_insert_with(|| HashMap::from([("before", vec![]), ("after", vec![])]))
            .get_mut("after")
            .unwrap()
            .push(first);
    }

    for line in data {
        let mut numbers: Vec<i32> = parse_pages(line);
        let mut is_valid: bool = true;

        for (index, number) in numbers.iter().enumerate() {
            let before_number = &numbers[..index];
            let after_number = &numbers[index + 1..];

            if let Some(rules) = validation_rules.get(&number) {
                if let Some(after_rules) = rules.get("after") {
                    if after_number.iter().any(|n| after_rules.contains(n)) {
                        is_valid = false;
                        break;
                    }
                }
                if let Some(before_rules) = rules.get("before") {
                    if before_number.iter().any(|n| before_rules.contains(n)) {
                        is_valid = false;
                        break;
                    }
                }
            }
        }
        if !is_valid {
            numbers.sort_by(|&a, &b| {
                if let Some(a_rules) = validation_rules.get(&a) {
                    let a_before = a_rules.get("before").unwrap();
                    let a_after = a_rules.get("after").unwrap();

                    if a_after.contains(&b) {
                        return std::cmp::Ordering::Greater
                    } else if a_before.contains(&b) {
                        return std::cmp::Ordering::Less
                    } else {
                        return std::cmp::Ordering::Equal
                    }
                } else if let Some(b_rules) = validation_rules.get(&b) {
                    let b_before = b_rules.get("before").unwrap();
                    let b_after = b_rules.get("after").unwrap();

                    if b_after.contains(&a) {
                        return std::cmp::Ordering::Less
                    } else if b_before.contains(&a) {
                        return std::cmp::Ordering::Greater
                    } else {
                        return std::cmp::Ordering::Equal
                    }
                }
                std::cmp::Ordering::Equal
            });

            let number_in_the_middle = numbers.len() / 2;
            result += numbers[number_in_the_middle];
        }
    }

    result
}

fn parse_validation_rule(line: String) -> (i32, i32) {
    let mut rules = line
        .split("|")
        .map(|part| part.parse::<i32>().expect("NaN LoL"));
    (rules.next().unwrap(), rules.next().unwrap())
}

fn parse_pages(line: String) -> Vec<i32> {
    line.split(",")
        .map(|num| num.parse::<i32>().expect("NaN LoL"))
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input_data = vec![
            "47|53".to_string(),
            "97|13".to_string(),
            "97|61".to_string(),
            "97|47".to_string(),
            "75|29".to_string(),
            "61|13".to_string(),
            "75|53".to_string(),
            "29|13".to_string(),
            "97|29".to_string(),
            "53|29".to_string(),
            "61|53".to_string(),
            "97|53".to_string(),
            "61|29".to_string(),
            "47|13".to_string(),
            "75|47".to_string(),
            "97|75".to_string(),
            "47|61".to_string(),
            "75|61".to_string(),
            "47|29".to_string(),
            "75|13".to_string(),
            "53|13".to_string(),
            "".to_string(),
            "75,47,61,53,29".to_string(),
            "97,61,53,29,13".to_string(),
            "75,29,13".to_string(),
            "75,97,47,61,53".to_string(),
            "61,13,29".to_string(),
            "97,13,75,29,47".to_string(),
        ];

        let result = run(input_data.into_iter());

        assert_eq!(result, 123);
    }
}
