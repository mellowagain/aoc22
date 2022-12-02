fn main() {
    let input = include_str!("input.txt");

    let strategies: Vec<_> = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(enemy, me)| (enemy.chars().next().unwrap(), me.chars().next().unwrap()))
        .collect();

    let a: i32 = strategies
        .iter()
        .map(|(enemy, me)| (Shape::from(*enemy), Shape::from(*me)))
        .map(|(enemy, me)| calculate_result(&enemy, &me))
        .map(|result| i32::from(result))
        .sum();

    let b: i32 = strategies
        .iter()
        .map(|(enemy, result)| {
            (
                Shape::from(*enemy),
                calculate_shape(&Shape::from(*enemy), &Result::from(result)),
            )
        })
        .map(|(enemy, me)| calculate_result(&enemy, &me))
        .map(|result| i32::from(result))
        .sum();

    println!("part a: {}", a);
    println!("part b: {}", b);
}

#[derive(Debug, Clone)]
#[repr(i32)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl From<char> for Shape {
    fn from(input: char) -> Self {
        match input {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissor,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Result {
    Win(Shape),
    Loss(Shape),
    Draw(Shape),
}

impl From<Result> for i32 {
    fn from(result: Result) -> Self {
        match result {
            Result::Win(shape) => 6 + (shape as i32),
            Result::Loss(shape) => 0 + (shape as i32),
            Result::Draw(shape) => 3 + (shape as i32),
        }
    }
}

impl From<&char> for Result {
    fn from(input: &char) -> Self {
        // don't care about what we actually supply here in the argument inside `Result`
        match *input {
            'X' => Result::Loss(Shape::Rock),
            'Y' => Result::Draw(Shape::Rock),
            'Z' => Result::Win(Shape::Rock),
            _ => unreachable!(),
        }
    }
}

fn calculate_result(enemy: &Shape, me: &Shape) -> Result {
    match (enemy, me) {
        (Shape::Rock, Shape::Paper) => Result::Win(Shape::Paper),
        (Shape::Rock, Shape::Scissor) => Result::Loss(Shape::Scissor),
        (Shape::Paper, Shape::Rock) => Result::Loss(Shape::Rock),
        (Shape::Paper, Shape::Scissor) => Result::Win(Shape::Scissor),
        (Shape::Scissor, Shape::Rock) => Result::Win(Shape::Rock),
        (Shape::Scissor, Shape::Paper) => Result::Loss(Shape::Paper),
        (_, me) => Result::Draw(me.clone()),
    }
}

fn calculate_shape(enemy: &Shape, result: &Result) -> Shape {
    match result {
        Result::Win(_) => match enemy {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissor,
            Shape::Scissor => Shape::Rock,
        },
        Result::Loss(_) => match enemy {
            Shape::Rock => Shape::Scissor,
            Shape::Paper => Shape::Rock,
            Shape::Scissor => Shape::Paper,
        },
        Result::Draw(_) => enemy.clone(),
    }
}
