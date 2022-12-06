use iter_columns::prelude::ColumnsTrait;

fn main() {
    let input: Vec<_> = include_str!("input.txt").lines().collect();

    println!("part a: {}", do_the_thing(&input, true));
    println!("part b: {}", do_the_thing(&input, false));
}

fn do_the_thing(input: &Vec<&str>, reverse: bool) -> String {
    let mut state: Vec<_> = input[..8]
        .iter()
        .columns()
        .skip(1)
        .step_by(4)
        .map(|bytes| String::from_utf8(bytes.into_iter().copied().collect()).unwrap())
        .map(|string| string.trim().to_string())
        .map(|string| string.chars().collect::<Vec<_>>())
        .collect();

    for instruction in &input[10..] {
        let instruction = &(instruction
            .replacen(" from ", ",", 1)
            .replacen(" to ", ",", 1))[5..];

        let mut instruction = instruction.splitn(3, ',');

        let amount: i32 = instruction.next().unwrap().parse().unwrap();
        let from: i32 = instruction.next().unwrap().parse().unwrap();
        let to: i32 = instruction.next().unwrap().parse().unwrap();

        let mut moving_crates: Vec<_> = {
            let from_crates = &mut state[(from - 1) as usize];

            from_crates.drain(0..amount as usize).collect()
        };

        if reverse {
            moving_crates.reverse();
        }

        let to_crates = &mut state[(to - 1) as usize];
        to_crates.splice(0..0, moving_crates.into_iter());
    }

    state.iter().map(|stack| stack.first().unwrap()).collect()
}
