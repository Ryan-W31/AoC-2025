use std::{fs, io::Read};

use itertools::Itertools;

fn main() {
    let mut buf = String::new();
    fs::File::open("./input.txt").unwrap().read_to_string(&mut buf).unwrap();

    let mut points: Vec<(i64, i64)> = Vec::new();
    for line in buf.lines() {
        let coords: Vec<i64> = line.trim().split(",").map(|n| n.parse::<i64>().unwrap()).collect();
        points.push((coords[0], coords[1]));
    }

    part1(points.clone());
    part2(points);
}



fn part1(points: Vec<(i64, i64)>) {
    let mut max_area = 0;
    for (&a, &b) in points.iter().tuple_combinations() {
            let area = calculate_area(a, b);
            if area > max_area {
                max_area = area;
            }
        }

    println!("Part 1 Ans: {}", max_area);
}

fn part2(points: Vec<(i64, i64)>) {
    let edges = get_total_poly_edges(points.clone());

    let mut max_area = 0;
    for (&a, &b) in points.iter().tuple_combinations() {
        let area = calculate_area(a, b);
        if area <= max_area {
            continue;
        }

        let (min_x, max_x) = if a.0 < b.0 {
            (a.0, b.0)
        } else {
            (b.0, a.0)
        };

        let (min_y, max_y) = if a.1 < b.1 {
            (a.1, b.1)
        } else {
            (b.1, a.1)
        };

        if is_inside(&edges, min_x, min_y, max_x, max_y) {
            max_area = area;
        }
    }

    println!("Part 2 Ans: {}", max_area);
}

fn get_total_poly_edges(points: Vec<(i64, i64)>) -> Vec<(i64, i64, i64, i64)> {
    let mut edges = Vec::with_capacity(points.len());
    for i in 0..points.len() {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % points.len()];
        edges.push((x1.min(x2), y1.min(y2), x1.max(x2), y2.max(y2)));
    }

    edges
}

fn is_inside(edges: &Vec<(i64, i64, i64, i64)>, min_x: i64, min_y: i64, max_x: i64, max_y: i64) -> bool {
    for edge in edges {
        let &(edge_min_x, edge_min_y, edge_max_x, edge_max_y) = edge;
        if min_x < edge_max_x && min_y < edge_max_y && max_x > edge_min_x && max_y > edge_min_y {
            return false;
        }
    }

    true
}

fn calculate_area(a: (i64, i64), b: (i64, i64)) -> i64 {
    let width = b.0.max(a.0) + 1 - b.0.min(a.0);
    let height = b.1.max(a.1) + 1 - b.1.min(a.1);

    height * width
}
