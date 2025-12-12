use std::{fs::File, io::{self, BufRead}, path::Path};

fn main() {
    let path = Path::new("./input.txt");
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut ranges: Vec<String> = Vec::new();

    for line in reader.lines() {
        let text = line.unwrap(); 
        ranges.extend(text.trim().split(',').map(|s| s.to_string()));
    }

    let mut res: i64 = 0;
    for range in ranges {
        let nums: Vec<i64> = range.split('-').map(|s| s.parse::<i64>().unwrap()).collect();
        let mut curr = nums[0];
        let max = nums[1];

        while curr <= max {
            let curr_as_string = curr.to_string();

            let str_len = curr_as_string.len();
            for i in 1..=str_len / 2 {
                if str_len % i == 0 {
                    let substring = &curr_as_string[..i];

                    let all_match = curr_as_string.as_bytes()
                        .chunks(i)
                        .all(|chunk| chunk == substring.as_bytes());

                    if all_match {
                        res += curr;
                        break;
                    }
                }
            }

            curr += 1;
        }
    }

    println!("Answer: {}", res)
}
