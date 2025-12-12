use std::{fs, io::Read};

fn main() {
    let mut buf = String::new();
    fs::File::open("./input.txt")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();

    let lines1 = buf.lines();
    let lines2 = buf.lines();
    let lines3 = buf.lines();

    let ranges: Vec<usize> = lines1
        .skip_while(|l| l.trim_start().chars().next().map_or(false, |c| c.is_ascii_digit()))
        .flat_map(|l| l.to_string().split(|c: char| c.is_ascii_graphic()).map(|s| s.len()).collect::<Vec<usize>>())
        .collect();
    
    let mut nums: Vec<Vec<String>> = lines2
        .take_while(|l| l.trim_start().chars().next().map_or(false, |c| c.is_ascii_digit()))
        .map(|line| {
            let mut offset = 0;

            ranges[1..].iter().map(|&len| {
                let end = offset + len + 1;
                let slice = line.get(offset..end).unwrap_or(""); 

                offset = end;
                slice.replace(" ", "-")
            }).collect::<Vec<String>>()
        })
        .collect();

    let ops: Vec<char> = lines3
        .skip_while(|l| l.trim_start().chars().next().map_or(false, |c| c.is_ascii_digit()))
        .flat_map(|l| l.chars().filter(|c| c.is_ascii_graphic()).collect::<Vec<char>>())
        .collect();

    let mut ans = 0;
    for (i, op) in ops.iter().enumerate() {
        ans += math2(i, ranges[i + 1], &mut nums, *op == '+');
    }

    println!("Ans: {}", ans);
}

fn math1(idx: usize, nums: &Vec<Vec<String>>, is_add: bool) -> i64 {
    let rows: usize = nums.len();

    let mut total = 1;
    if is_add {
        total = 0
    }
    for row in 0..rows {
        let num = nums[row][idx].replace("-", "").parse::<i64>().unwrap();
        if is_add {
            total += num;
        } else {
            total *= num;
        }
    }

    total
}

fn math2(idx: usize, range: usize, nums: &mut Vec<Vec<String>>, is_add: bool) -> i64 {
    let rows: usize = nums.len();

    let mut total = 1;
    if is_add {
        total = 0;
    }

    for i in 0..=range {
        let mut inter_total = 0;
        for row in 0..rows {
            let ch = nums[row][idx].chars().nth(i).unwrap();

            inter_total += if ch == '-' {
                0
            } else {
                inter_total *= 10;
                ch.to_digit(10).unwrap() as i64
            };
        }

        if is_add {
            total += inter_total;
        } else if inter_total != 0 {
            total *= inter_total;
        }
    }

    total
}

