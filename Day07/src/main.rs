use std::{collections::VecDeque, fs, io::Read};

fn main() {
    let mut buf = String::new();
    fs::File::open("./input.txt")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();

    let mut grid: Vec<Vec<char>> = buf.lines()
        .map(|l| l.trim().chars().collect())
        .collect::<Vec<Vec<char>>>();

    //let ans = bfs(&mut grid);
    let rows = grid.len();
    let cols = grid[0].len();
    let start_col = cols / 2;
    let mut cache: Vec<Vec<Option<i64>>> = vec![vec![None; cols]; rows];

    let ans = dfs(&mut grid, 0, start_col, &mut cache);
    println!("Ans: {}", ans);
}

fn bfs(grid: &mut Vec<Vec<char>>) -> i64 {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    q.push_back((0, grid[0].len() / 2));

    let mut ans = 0;
    while !q.is_empty() {
        let (curr_row, curr_col) = q.pop_front().unwrap();

        let new_row = curr_row + 1;
        if new_row >= grid.len() || grid[new_row][curr_col] == '|' {
            continue;
        }

        if grid[new_row][curr_col] == '.' {
            grid[new_row][curr_col] = '|';
            q.push_back((new_row, curr_col));
        } else {
            if curr_col > 0 {
                let left_col = curr_col - 1;
                grid[new_row][left_col] = '|';
                q.push_back((new_row, left_col));
            }

            if curr_col < grid[0].len() - 2 {
                let right_col = curr_col + 1;
                grid[new_row][right_col] = '|';
                q.push_back((new_row, right_col));
            }

            ans += 1;
        }
    }

    ans
}

fn dfs(grid: &mut Vec<Vec<char>>, row: usize, col: usize, cache: &mut Vec<Vec<Option<i64>>>) -> i64  {
    let rows = grid.len();
    let cols = grid[0].len();

    if col >= cols {
        return 0;
    }

    let new_row = row + 1;
    if row == rows || new_row == rows{
        return 1;
    }

    if let Some(cached_res) = cache[row][col] {
        return cached_res;
    }

    let mut ans = 0;
    if grid[new_row][col] == '.' {
        grid[new_row][col] = '|';
        ans += dfs(grid, new_row, col, cache);
        grid[new_row][col] = '.';
    } else {
        if col > 0 {
            let left_col = col - 1;
            grid[new_row][left_col] = '|';
            ans += dfs(grid, new_row, left_col, cache);
            grid[new_row][left_col] = '.';
        }

        if col + 1 < cols {
            let right_col = col + 1;
            grid[new_row][right_col] = '|';
            ans += dfs(grid, new_row, right_col, cache);
            grid[new_row][right_col] = '.';
        }
    }

    cache[row][col] = Some(ans);
    ans
}
