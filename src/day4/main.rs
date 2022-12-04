fn main() {
    let input: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(a, b)| (a.split_once('-').unwrap(), b.split_once('-').unwrap()))
        .map(|((a_start, a_end), (b_start, b_end))| {
            (
                (
                    a_start.parse::<i32>().unwrap(),
                    a_end.parse::<i32>().unwrap(),
                ),
                (
                    b_start.parse::<i32>().unwrap(),
                    b_end.parse::<i32>().unwrap(),
                ),
            )
        })
        .collect();

    let a: i32 = input
        .iter()
        .map(|((a_start, a_end), (b_start, b_end))| {
            ((a_start <= b_start && a_end >= b_end) || (b_start <= a_start && b_end >= a_end))
                as i32
        })
        .sum();

    let b: i32 = input
        .iter()
        .map(|((a_start, a_end), (b_start, b_end))| (a_start <= b_end && b_start <= a_end) as i32)
        .sum();

    println!("part a: {}", a);
    println!("part b: {}", b);
}
