use std::{fs::File, io::{self, BufRead}, path::Path};

fn main() {
    let path = Path::new("./input.txt");
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut ans = 0;
    for line in reader.lines() {
        let text = line.unwrap(); 

        let chars: Vec<char> = text.chars().collect();
        let n = chars.len();
        let mut stack: Vec<char> = Vec::with_capacity(12);

        for (i, &digit) in chars.iter().enumerate() {
            while let Some(&top) = stack.last() {
                let remaining_chars = n - 1 - i;
                if digit > top && stack.len() + remaining_chars >= 12 {
                    stack.pop();
                } else {
                    break;
                }
            }

            if stack.len() < 12 {
                stack.push(digit);
            }
        }

        let new_num: String = stack.iter().collect();
        ans += new_num.parse::<i64>().unwrap();
    }
    println!("Ans: {}", ans);
}
