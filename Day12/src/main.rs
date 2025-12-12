use std::{fs, io::Read};

use itertools::Itertools;

fn main() {
    let mut buf = String::new();
    fs::File::open("./input.txt").unwrap().read_to_string(&mut buf).unwrap();

    let lines1 = buf.lines();
    let lines2 = buf.lines();

    let presents: Vec<(Vec<usize>, usize)> = lines1
        .filter(|s| s.starts_with(".") || s.starts_with("#"))
        .collect_vec()
        .chunks(3)
        .map(|chunk| {
            let vec = chunk.iter()
                .flat_map(|s| {
                    s.chars()
                        .map(|c| {
                            match c {
                                '#' => 1,
                                '.' => 0,
                                _ => panic!("unexpected char {}", c),
                            }
                        })
                })
                .collect_vec();
            let area = vec.iter().sum();
            (vec, area)
        })
        .collect_vec();

    let grids: Vec<(Vec<usize>, Vec<usize>)> = lines2
        .filter(|s| s.contains("x"))
        .collect_vec()
        .iter()
        .map(|s| {
            let split = s.split(":").collect_vec();
            let size: Vec<usize> = split[0].split("x").collect_vec().iter().map(|n| n.parse::<usize>().unwrap()).collect_vec();
            let present_counts: Vec<usize> = split[1].trim_start().split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect_vec();
            (size, present_counts)
        }).collect_vec();

    let valid_grids = grids.iter().filter(|g| is_valid_grid(g, &presents)).map(|g| g.to_owned()).collect_vec();
    // let presents_with_counts = get_presents(&presents, &valid_grids);

    // -_- All the valid grids work
    println!("Valid grids: {} / {}", valid_grids.len(), grids.len());

    //println!("presents_with_counts: {:?}", presents_with_counts[0]);
}

fn get_presents(presents: &Vec<(Vec<usize>, usize)>, grids: &Vec<(Vec<usize>, Vec<usize>)>) -> Vec<Vec<((Vec<usize>, usize), usize)>> {
    let mut presents_with_counts: Vec<Vec<((Vec<usize>, usize), usize)>> = Vec::new();
    for grid in grids {
        let counts = &grid.1;

        let mut p = Vec::new();
        for (i, &count) in counts.iter().enumerate() {
            p.push((presents.get(i).unwrap().clone(), count));
        }

        p.sort_by(|a, b| {
            if b.0.1 == a.0.1 {
                return b.1.cmp(&a.1);
            }

            b.0.1.cmp(&a.0.1)
        });

        presents_with_counts.push(p);
    }

    presents_with_counts
}

fn is_valid_grid(g: &(Vec<usize>, Vec<usize>), presents: &Vec<(Vec<usize>, usize)>) -> bool {
    let (rows, cols) = (g.0[0], g.0[1]);
    let counts = &g.1;

    let mut total_area = 0;
    for (i, count) in counts.iter().enumerate() {
        total_area += count * presents[i].1;
    }

    if rows * cols < total_area {
        return false;
    }

    true
}

// fn part1(presents: Vec<((Vec<usize>, usize), usize)>, grid: &mut Vec<Vec<usize>>, idx: usize) -> bool {
//
//     for present in presents {
//         if present.1 == 0 {
//             continue;
//         }
//
//         let present_area = present.0.1;
//         let grid_area = grid.iter().map(|r| r.iter().sum::<usize>()).sum::<usize>();
//
//         if present_area > grid_area {
//             return false;
//         }
//     }
//
//     false
// }
