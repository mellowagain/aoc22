static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let input = include_str!("input.txt");

    let a: i32 = input.lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(first, second)| {
            for c in first.chars() {
                if second.find(c).is_some() {
                    return (ALPHABET.find(c).unwrap() as i32) + 1;
                }
            }

            0
        })
        .sum();

    let lines: Vec<_> = input.lines().collect();
    let b: i32 = lines.iter()
        .enumerate()
        .step_by(3)
        .map(|(index, first)| {
            let second = lines[index + 1];
            let third = lines[index + 2];

            for character in first.chars() {
                if second.contains(character) && third.contains(character) {
                    return (ALPHABET.find(character).unwrap() as i32) + 1;

                }
            }

            0
        })
        .sum();

    println!("part a: {}", a);
    println!("part b: {}", b);
}
