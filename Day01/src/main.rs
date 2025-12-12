use std::{fs::File, io::{self, BufRead}, path::Path};

fn main() {
    let path = Path::new("./input.txt");
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut nums: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let text = line.unwrap(); 

        let idx = text.find(|c: char| c.is_digit(10)).unwrap();

        let (letter, number) = text.split_at(idx);
        if letter == "L" {
            nums.push(-1 * number.trim().parse::<i32>().unwrap());
        } else {
            nums.push(1 * number.trim().parse::<i32>().unwrap());
        }
    }

    let mut spin_num: i32 = 50;
    let mut ans = 0;
    for num in &nums {
        let res = spin_num + num;

        spin_num = res.rem_euclid(100);

        if spin_num == 0 {
            ans += 1;
        }
    }
    
    for num in nums {
        ans += (num.abs() / 100) as usize;

        let rem = num.abs() % 100;

        let dir = if num > 0 { 1 } else { -1 };

        for _ in 0..rem {
            spin_num += dir;

            if spin_num > 99 {
                spin_num = 0;
            } else if spin_num < 0 {
                spin_num = 99;
            }

            if spin_num == 0 {
                ans += 1;
            }
        }
    }

    println!("Answer: {}", ans)
}
