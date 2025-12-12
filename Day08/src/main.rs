use std::{cmp::Ordering, fs, io::Read};

#[derive(Debug)]
pub struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, mut curr: usize) -> usize {
        let mut root = curr;
        while root != self.parent[root] {
            root = self.parent[root];
        }

        while curr != root {
            let next = self.parent[curr];
            self.parent[curr] = root;
            curr = next;
        }

        root
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return false;
        }

        if self.size[root_a] < self.size[root_b] {
            self.parent[root_a] = root_b;
            self.size[root_b] += self.size[root_a];
        } else {
            self.parent[root_b] = root_a;
            self.size[root_a] += self.size[root_b];
        }

        true
    }

    pub fn get_set_size(&mut self, curr: usize) -> usize {
        let root = self.find(curr);
        self.size[root]
    }

    pub fn check_size_total(&mut self) -> bool {
        for size in &self.size {
            if *size == self.size.len() {
                return true;
            }
        }

        false
    }
}


fn main() {
    let mut buf = String::new();
    fs::File::open("./input.txt").unwrap().read_to_string(&mut buf).unwrap();

    let mut points: Vec<(usize, (i64, i64, i64))>= Vec::new();

    let mut count = 0;
    for (i, line) in buf.lines().enumerate() {
        let nums = line.trim().split(",").map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let point = (nums[0], nums[1], nums[2]);

        
        points.push((i, point));
        count += 1;
    }

    let mut ds = DisjointSet::new(count);

    let mut distances: Vec<(i64, (usize, usize))> = Vec::new();
    for point_a in points.clone() {
        let clone = points.clone();
        for point_b in &clone[point_a.0..] {
            if point_a.0 == point_b.0 {
                continue;
            }

            let euc_dist = get_euc_distance(point_a.1, point_b.1);
            distances.push((euc_dist, (point_a.0, point_b.0)));
        }
    }
    
    distances.sort_by(|a, b| {
        let euc_dist_diff = a.0.cmp(&b.0);
        if euc_dist_diff == Ordering::Equal {
            return a.1.1.cmp(&b.1.1);
        }
        euc_dist_diff
    });

    // part 1
    // for i in 0..1000 {
    //     let dist = distances[i];
    //
    //     let (point_a, point_b) = dist.1;
    //     ds.union(point_a, point_b);
    // }

    for dist in distances {
        let (point_a, point_b) = dist.1;
        ds.union(point_a, point_b);
        if ds.check_size_total() {
            let x_total = points.get(point_a).unwrap().1.0 * points.get(point_b).unwrap().1.0;
            println!("Part 2 Ans: {}", x_total);
            break;
        }
    }

    // part 1
    // ds.size.sort_by(|a, b| b.cmp(a));
    // let sizes: Vec<_> = ds.size.iter().take(3).collect();
    //
    // let mut ans: i64 = 1;
    // for size in sizes {
    //     ans *= *size as i64;
    // }
    //
    // println!("Part 1 Ans: {}", ans);
}

fn get_euc_distance(a: (i64, i64, i64), b: (i64, i64, i64)) -> i64 {
    let x_diff = (a.0 - b.0).pow(2);
    let y_diff = (a.1 - b.1).pow(2);
    let z_diff = (a.2 - b.2).pow(2);

    (x_diff + y_diff + z_diff).isqrt()
}
