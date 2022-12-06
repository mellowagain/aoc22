use fancy_regex::Regex;

fn main() {
    let input = include_str!("input.txt").trim();

    println!("part a: {}", check_first_non_repeating(input, 4));
    println!("part b: {}", check_first_non_repeating(input, 14));
}

fn check_first_non_repeating(input: &str, amount: usize) -> usize {
    // https://stackoverflow.com/a/1660739/11494565
    let regex = Regex::new(r#"(.).*\1"#).unwrap();

    for (pos, _) in input.chars().enumerate().skip(amount - 1) {
        let part = &input[pos - (amount - 1)..pos + 1];

        if !regex.is_match(part).unwrap() {
            return pos + 1;
        }
    }

    0
}
