use std::{cmp::max, fs, io::Read};

fn main() {
    let mut buf = String::new();
    fs::File::open("./input.txt")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();

    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut ingredients: Vec<usize> = Vec::new();

    let mut max = 0;
    let mut is_ranges = true;

    for line in buf.lines() {
        if line.is_empty() {
            is_ranges = false;
            continue;
        }

        if is_ranges {
            let mut parts = line.trim().split("-");
            let first = parts.next().unwrap().trim().parse::<usize>().unwrap();
            let second = parts.next().unwrap().trim().parse::<usize>().unwrap();

            if second > max {
                max = second;
            }

            ranges.push((first, second));
        } else {
            ingredients.push(line.trim().parse::<usize>().unwrap());
        }
    }

    let intervals = merge_intervals(&mut ranges);
    let count = ingredients1(&intervals, &ingredients);
    let total = ingredients2(&intervals);
    println!("Ans: {}", count);
    println!("Total: {}", total);
}

fn merge_intervals(ranges: &mut Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merged: Vec<(usize, usize)> = Vec::new();
    let mut curr_start = ranges[0].0;
    let mut curr_end = ranges[0].1;

    for range in ranges.into_iter().skip(1) {
        let next_start = range.0;
        let next_end = range.1;

        if curr_end >= next_start || curr_end + 1 == next_start {
            curr_end = max(curr_end, next_end);
        } else {
            merged.push((curr_start, curr_end));
            curr_start = next_start;
            curr_end = next_end;
        }
    }

    merged.push((curr_start, curr_end));
    merged
}

fn ingredients1(intervals: &Vec<(usize, usize)>, ingredients: &Vec<usize>) -> usize {
    let mut count = 0;

    for &ing in ingredients {
        let idx = intervals.partition_point(|range| range.0 <= ing);

        if idx > 0 {
            let (start, end) = intervals[idx - 1];

            if ing >= start && ing <= end {
                count += 1;
            }
        }
    }

    count
}

fn ingredients2(intervals: &Vec<(usize, usize)>) -> usize {
    let mut count = 0;

    for &interval in intervals {
        count += interval.1 - interval.0 + 1;
    }

    count
}
