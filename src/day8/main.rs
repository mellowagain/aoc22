fn main() {
    let grid: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|a| {
            a.chars()
                .map(|tree| tree.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    println!("part a: {}", part_a(&grid));
    println!("part b: {}", part_b(&grid));
}

fn part_a(grid: &Vec<Vec<u32>>) -> usize {
    // edges (row + column length - 4 for the double counted edges)
    let mut visible = (grid.len() * 2) + (grid[0].len() * 2) - 4;

    // skip first and last row
    let mut rows = grid.iter().enumerate().skip(1);
    rows.next_back();

    for (row_index, row) in rows {
        let mut columns = row.iter().enumerate().skip(1);
        columns.next_back();

        for (column_index, column) in columns {
            let left = row[0..column_index].iter().max().unwrap_or(&0);
            let right = row[column_index + 1..row.len()].iter().max().unwrap_or(&0);
            let top = grid[0..row_index]
                .iter()
                .map(|a| a[column_index])
                .max()
                .unwrap_or(0);
            let bottom = grid[row_index + 1..grid.len()]
                .iter()
                .map(|a| a[column_index])
                .max()
                .unwrap_or(0);

            if left < column || right < column || top < *column || bottom < *column {
                visible += 1;
            }
        }
    }

    visible
}

fn part_b(grid: &Vec<Vec<u32>>) -> usize {
    let mut top_score = 0;

    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            // .take_while does not yield the last element that it gets the `false` on
            // but for this challenge we actually want that one to count too, so we need
            // a way to check if it's valid and then retroactively add it
            let mut valid = false;

            let left = row[0..column_index]
                .iter()
                .rev()
                .take_while(|tree| {
                    valid = *tree < column;
                    valid
                })
                .count()
                + !valid as usize;

            let right = row[column_index + 1..row.len()]
                .iter()
                .take_while(|tree| {
                    valid = *tree < column;
                    valid
                })
                .count()
                + !valid as usize;

            let top = grid[0..row_index]
                .iter()
                .rev()
                .map(|a| a[column_index])
                .take_while(|tree| {
                    valid = tree < column;
                    valid
                })
                .count()
                + !valid as usize;

            let bottom = grid[row_index + 1..grid.len()]
                .iter()
                .map(|a| a[column_index])
                .take_while(|tree| {
                    valid = tree < column;
                    valid
                })
                .count()
                + !valid as usize;

            let score = left * right * top * bottom;

            if score > top_score {
                top_score = score;
            }
        }
    }

    top_score
}
