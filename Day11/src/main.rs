use std::{collections::{HashMap, VecDeque}, fs, io::Read};

use itertools::Itertools;

fn main() {
    let mut buf = String::new();
    fs::File::open("./input.txt").unwrap().read_to_string(&mut buf).unwrap();

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let lines = buf.lines();

    for line in lines {
        let devices = line.split_whitespace().collect_vec();
        let devices = devices.iter().map(|s| s.to_string()).collect_vec();

        let from_trimmed = devices[0].replace(":", "");
        graph.insert(from_trimmed, devices[1..].to_vec());
    }

    // let ans1 = part1(&mut graph);
    // println!("Part 1 Ans: {}", ans1);


    let mut cache: HashMap<(String, bool, bool), i64> = HashMap::new();
    let ans2 = part2(&graph, "svr".to_string(), false, false, &mut cache);
    println!("Part 2 Ans: {}", ans2);
}

fn part1(graph: &mut HashMap<String, Vec<String>>) -> i64 {
    let mut q: VecDeque<String> = VecDeque::new();
    q.push_front("you".to_string());

    let mut count = 0;
    while let Some(from) = q.pop_front() {
        let tos = graph.get(&from).unwrap();

        if tos.is_empty() {
            continue;
        }

        // assumes no cycles
        for to in tos {
            if to == "out" {
                count += 1
            } else {
                q.push_back(to.clone());
            }
        }
    }

    count
}

fn part2(graph: &HashMap<String, Vec<String>>, curr: String, fft_checkpoint: bool, dac_checkpoint: bool, cache: &mut HashMap<(String, bool, bool), i64>) -> i64 {
    let cache_key = (curr.to_string(), fft_checkpoint, dac_checkpoint);
    if let Some(&count) = cache.get(&cache_key) {
        return count;
    }

    if curr == "out" {
        return if fft_checkpoint && dac_checkpoint { 1 } else { 0 };
    }

    let new_fft = fft_checkpoint || (curr == "fft");
    let new_dac = dac_checkpoint || (curr == "dac");

    let mut count = 0;

    if let Some(neighbors) = graph.get(&curr) {
        for to in neighbors {
            count += part2(graph, to.clone(), new_fft, new_dac, cache);
        }
    }

    cache.insert(cache_key, count);
    count
}
