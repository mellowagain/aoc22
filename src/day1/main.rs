use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    let mut vec: Vec<i32> = input.split("\n\n")
        .map(|elf| elf.lines().map(|calories| calories.parse::<i32>().unwrap()).sum())
        .collect();

    let b: i32 = (0..3).map(|a| vec.remove(vec.iter().position_max().unwrap())).sum();
    let a = vec.into_iter().max().unwrap();

    println!("part a: {}", a);
    println!("part b: {}", b);
}
