use std::{fs::File, io::{self, BufRead}, path::Path};

fn main() {
    let path = Path::new("./input.txt");
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let text = line.unwrap(); 
        grid.push(text.trim().chars().collect());
    }

    let mut ans = 0;

    loop {
        let mut inter_ans = 0;
        let mut taken_rolls: Vec<(usize, usize)> = Vec::new();

        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, ch) in row.iter().enumerate() {
                if *ch == '@' {
                    let res = forklift1(&grid, (row_idx, col_idx));
                    inter_ans += res;
                    if res > 0 {
                        taken_rolls.push((row_idx, col_idx));
                    }
                }
            }
        }

        if inter_ans == 0 {
            break;
        }

        remove_taken(&mut grid, taken_rolls);


        ans += inter_ans;
    }

    println!("Ans: {}", ans);
}

fn forklift1(grid: &Vec<Vec<char>>, curr: (usize, usize)) -> i32 {
    let moves = vec!(0, 1, 0, -1, -1, 1, 1, -1, 0);

    let mut count = 0;
    for i in 0..8 {
        let row = curr.0 as i32 + moves[i];
        let col = curr.1 as i32 + moves[i + 1];

        if !is_valid(grid, (row, col)) {
            continue;
        }

        if grid[row as usize][col as usize] == '@' {
            count += 1;

            if count > 3 {
                return 0;
            }
        }

    }

    1
}

fn remove_taken(grid: &mut Vec<Vec<char>>, taken: Vec<(usize, usize)>) {
    for (row, col) in taken {
        grid[row][col] = '.';
    }
}

fn is_valid(grid: &Vec<Vec<char>>, curr: (i32, i32)) -> bool {
    if curr.0 < 0 || curr.1 < 0 || curr.0 >= grid.len() as i32 || curr.1 >= grid[0].len() as i32 {
        return false
    }
    return true
}
