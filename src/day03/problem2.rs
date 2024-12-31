use regex::Regex;

pub fn run<I>(input_data: I) -> i32
where
    I: Iterator<Item = String>,
{
    let mut result: i32 = 0;
    let re = Regex::new(
        r"(?x)                      # Now this is a cool feature, free-spacing mode FTW
        (?P<enabled>do\(\))         # I expect either one of the logic operators to match
        |(?P<disabled>don't\(\))    # or both factor groups so I can loop over all capture 
        |mul\(                      # groups and control the flow 
        (?P<factor1>\d+),
        (?P<factor2>\d+)
        \)
        ",
    )
    .expect("I messed up the regex");
    let mut is_enabled = true;

    for line in input_data {
        for captures in re.captures_iter(&line) {
            if let Some(_) = captures.name("disabled") {
                is_enabled = false;
                continue;
            }
            if let Some(_) = captures.name("enabled") {
                is_enabled = true;
                continue;
            }
            if is_enabled {
                let factor1: i32 = captures["factor1"]
                    .parse()
                    .expect(&format!("Could not parse {}", &captures["factor1"]));
                let factor2: i32 = captures["factor2"]
                    .parse()
                    .expect(&format!("Could not parse {}", &captures["factor2"]));
                result += factor1 * factor2;
            }
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
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string(),
        ];

        let result = run(input_data.into_iter());

        assert_eq!(result, 48);
    }
}
