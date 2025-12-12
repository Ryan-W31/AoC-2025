use std::{collections::{HashSet, VecDeque}, fs, io::Read, iter::{Inspect, Sum, zip}};

use itertools::Itertools;
use z3::{Optimize, SatResult, ast::Int};

fn main() {
    let mut buf = String::new();
    fs::File::open("./input.txt").unwrap().read_to_string(&mut buf).unwrap();

    let lines1 = buf.lines();
    let lines2 = buf.lines();
    let lines3 = buf.lines();

    let indicators = lines1
        .map(|l| l.trim().split_whitespace().nth(0))
        .map(|s| s.unwrap().to_string())
        .map(|i| i.replace("[", "").replace("]", ""))
        .collect_vec();

    let buttons = lines2
        .map(|line| line.split_whitespace().collect_vec())
        .map(|b| b.clone().into_iter().skip(1).take(b.len().saturating_sub(2)).collect_vec())
        .map(|buttons| buttons.iter()
            .map(|b|b.chars().filter_map(|c| c.to_digit(10).map(|d| d as usize)).collect_vec())
            .collect_vec())
        .collect_vec();

    let joltages = lines3
        .map(|l| {
            l.trim()
                .split_whitespace()
                .last()
                .unwrap()
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|s| s.to_string().parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut ans = 0;
    for (btns, inds) in zip(buttons.clone(), indicators) {
        let len = inds.len();
        let start = ".".repeat(len);
        ans += part1(&btns, start, inds);

    }
    println!("Part 1 Ans: {}", ans);

    // Z3 implemetation
    ans = 0;
    let mut count = 0;
    for (btns, jlts) in zip(buttons.clone(), joltages) {
        ans += part2_z3_yuck(&jlts, &btns);
        count += 1;
    }
    println!("Part 2 Ans: {}", ans);
}

fn part1(buttons: &Vec<Vec<usize>>, start_state: String, target_state: String) -> usize {
    let mut visited: HashSet<String> = HashSet::new();
    let mut q: VecDeque<(String, usize)> = VecDeque::new();

    visited.insert(start_state.clone());
    q.push_back((start_state, 0));

    while let Some((curr_state, count)) = q.pop_front() {
        if curr_state == target_state {
            return count;
        }

        for button in buttons {
            let mut new_chars: Vec<char> = curr_state.chars().collect();
            for &idx in button {
                if new_chars[idx] == '.' {
                    new_chars[idx] = '#';
                } else {
                    new_chars[idx] = '.';
                }
            }

            let new_state: String = new_chars.into_iter().collect();

            if !visited.contains(&new_state) {
                visited.insert(new_state.clone());
                q.push_back((new_state, count + 1));
            }
        }
    }

    0
}

fn part2_z3_yuck(joltages: &Vec<usize>, buttons: &Vec<Vec<usize>>) -> usize {
    let num_buttons = buttons.len();

    let opt = Optimize::new();
    let total = Int::fresh_const("total");

    let presses: Vec<Int> = (0..num_buttons)
        .map(|idx| Int::fresh_const(&format!("x_{idx}")))
        .collect();

    presses.iter().for_each(|b| opt.assert(&b.ge(0)));

    for (pos, &target) in joltages.iter().enumerate() {
        let mut terms = Vec::new();

        for (idx, button) in buttons.iter().enumerate() {
            if button.contains(&pos) {
                terms.push(presses[idx].clone());
            }
        }
        let sum = Int::add(&terms.iter().collect::<Vec<&Int>>());
        opt.assert(&sum.eq(Int::from_u64(target as u64)));
    }

    opt.assert(&total.eq(Int::add(&presses)));
    opt.minimize(&total);

    match opt.check(&[]) {
        SatResult::Sat => opt
            .get_model()
            .unwrap()
            .eval(&total, true)
            .and_then(|t| t.as_u64())
            .unwrap() as usize,
        _              => panic!("No solution found"),
    }
}
