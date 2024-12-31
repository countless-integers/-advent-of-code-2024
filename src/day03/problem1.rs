use regex::Regex;

pub fn run<I>(input_data: I) -> i32
where
    I: Iterator<Item = String>,
{
    let mut result: i32 = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("I messed up the regex");

    for line in input_data {
        for captures in re.captures_iter(&line) {
            let factor1: i32 = captures[1].parse().expect(&format!("Could not parse {}", &captures[1]));
            let factor2: i32 = captures[2].parse().expect(&format!("Could not parse {}", &captures[2]));
            result += factor1 * factor2;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input_data = vec![
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string(),
        ];

        let result = run(input_data.into_iter());

        assert_eq!(result, 161);
    }
}
