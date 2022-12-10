use std::collections::HashMap;
use std::path::PathBuf;
use itertools::Itertools;

fn main() {
    let directories = parse_file_tree(include_str!("input.txt"));

    let a: i32 = directories
        .iter()
        .map(|(_, size)| size)
        .filter(|size| **size < 100000)
        .sum();

    let b = {
        let total = 70000000;
        let required = 30000000;

        let used = *directories.get(PathBuf::from("/").as_path()).unwrap();
        let free = total - used;
        let required_free = required - free;

        let smaller: Vec<_> = directories
            .iter()
            .map(|(_, size)| size)
            .filter(|size| **size > required_free)
            .sorted()
            .collect();

        **smaller.first().unwrap()
    };

    println!("part a: {}", a);
    println!("part b: {}", b);
}

fn parse_file_tree(input: &str) -> HashMap<PathBuf, i32> {
    let mut directories = HashMap::new();
    let mut cwd = PathBuf::from("/");

    for line in input.lines() {
        if let Some(command) = line.strip_prefix("$ ") {
            match &command[0..2] {
                "cd" => {
                    match &command[3..] {
                        "/" => {
                            cwd = PathBuf::from("/");
                        }
                        ".." => {
                            cwd.pop();
                        }
                        dir => {
                            cwd.push(dir);
                        }
                    }
                }
                "ls" => { /* ignored */ }
                _ => unreachable!(),
            }
        } else {
            let (size, _) = line.split_once(' ').unwrap();

            if size == "dir" {
                continue;
            }

            let size: i32 = size.parse().unwrap();

            // I peeked at evans solution and found this, I previously just did .entry on the `cwd`, not all ancestors
            for ancestor in cwd.ancestors() {
                *directories.entry(ancestor.to_path_buf()).or_insert(0) += size;
            }
        }
    }

    directories
}
