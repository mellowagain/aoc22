use std::collections::HashSet;

fn main() {
    let mut map: HashSet<(i32, i32)> = HashSet::new();

    let input = include_str!("input2.txt")
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(direction, amount)| {
            (
                direction.chars().nth(0).unwrap(),
                amount.parse::<i32>().unwrap(),
            )
        });

    let mut head_x = 0;
    let mut head_y = 0;
    let mut tail_x = 0;
    let mut tail_y = 0;

    for (direction, amount) in input {
        match direction {
            'L' => head_x -= amount,
            'R' => head_x += amount,
            'U' => head_y -= amount,
            'D' => head_y += amount,
            _ => unreachable!(),
        }

        if !is_adjacent(head_x, head_y, tail_x, tail_y) && (head_x != head_y || tail_x != tail_y) {
            // If the head is ever two steps directly up, down, left, or right from the tail,
            // the tail must also move one step in that direction so it remains close enough:
            if head_x == tail_x && head_y < tail_y {
                for _ in 0..(tail_y - head_y) - 1 {
                    tail_y -= 1;
                    map.insert((tail_x, tail_y));
                }
            } else if head_x == tail_x && head_y > tail_y {
                for _ in 0..(head_y - tail_y) - 1 {
                    tail_y += 1;
                    map.insert((tail_x, tail_y));
                }
            } else if head_x < tail_x && head_y == tail_y {
                for _ in 0..(tail_x - tail_x) - 1 {
                    tail_x -= 1;
                    map.insert((tail_x, tail_y));
                }
            } else if head_x > tail_x && head_y == tail_y {
                for _ in 0..(head_x - tail_x) - 1 {
                    tail_x += 1;
                    map.insert((tail_x, tail_y));
                }
            // Otherwise, if the head and tail aren't touching and aren't in the same row or column,
            // the tail always moves one step diagonally to keep up:
            } else if head_x > tail_x && head_y > tail_y {
                let diff_x = head_x - tail_x;
                let diff_y = head_y - tail_y;

                println!("delta1 x: {diff_x} y: {diff_y}");

                tail_x += 1;
                tail_y += 1;
            } else if head_x < tail_x && head_y > tail_y {
                let diff_x = tail_x - head_x;
                let diff_y = head_y - tail_y;
                println!("delta2 x: {diff_x} y: {diff_y}");

                tail_x -= 1;
                tail_y += 1;
            } else if head_x < tail_x && head_y < tail_y {
                let diff_x = tail_x - head_x;
                let diff_y = tail_y - head_y;
                println!("delta3 x: {diff_x} y: {diff_y}");

                tail_x -= 1;
                tail_y -= 1;
            } else if head_x > tail_x && head_y < tail_y {
                let diff_x = head_x - tail_x;
                let diff_y = tail_y - head_y;
                println!("delta4 x: {diff_x} y: {diff_y}");

                // head: 4 -4
                // tail: 3 0

                /* ***********
                   ***********
                   ***********
                   *****0**T**
                   ***********
                   ***********
                   ***********
                   *********H*
                   1 diagonal, 2 up

                   recv: delta4 x: 1 y: 4


                */

                tail_x += 1;
                tail_y -= 1;
            }
        }

        println!("head @ {head_x} {head_y} / tail @ {tail_x} {tail_y}");
        map.insert((tail_x, tail_y));
    }

    // 1623 too low
    println!("part a: {}", map.len());
}

fn is_adjacent(head_x: i32, head_y: i32, tail_x: i32, tail_y: i32) -> bool {
    if head_x == tail_x && (head_y - tail_y).abs() == 1 {
        // above or below
        true
    } else if head_y == tail_y && (head_x - tail_x).abs() == 1 {
        // left or right
        true
    } else if (head_x - tail_x).abs() == 1 && (head_y - tail_y).abs() == 1 {
        // diagonally
        true
    } else {
        false
    }
}
