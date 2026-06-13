use rand::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
fn d((x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> f64 {
    let x_dist = if x2 > x1 { x2 - x1 } else { x1 - x2 };
    let y_dist = if y2 > y1 { y2 - y1 } else { y1 - y2 };
    f64::from(x_dist.pow(2) + y_dist.pow(2)).sqrt()
}
mod kruskal {
    struct SetMember {
        parent: usize,
        size: usize,
    }
    fn find_parent(set: &mut Vec<SetMember>, x: usize) -> usize {
        let inwards: usize;
        {
            let node = &set[x as usize];
            if x == node.parent {
                return x;
            }
            inwards = find_parent(set, node.parent);
        }
        let mut_node = &mut set[x as usize];
        mut_node.parent = inwards;
        return inwards;
    }
    fn union(set: &mut Vec<SetMember>, x: usize, y: usize) -> bool {
        let mut px = find_parent(set, x);
        let mut py = find_parent(set, y);
        if px == py {
            return false;
        }
        if set[px as usize].size < set[py as usize].size {
            (px, py) = (py, px); //Union by Rank
        }
        set[py as usize].parent = set[x as usize].parent;
        set[px as usize].size += set[py as usize].size;
        true
    }
    fn find_closest(adj_mat: &Vec<Vec<f64>>, excluded: usize) -> (f64, f64) {
        let mut ret: (f64, f64) = (f64::INFINITY, f64::INFINITY);
        let n = adj_mat.len();
        for i in 0..n {
            if i == excluded {
                continue;
            }
            let curr = adj_mat[i][excluded];
            if curr < ret.0 {
                ret.0 = curr;
            }
        }
        for i in 0..n {
            if i == excluded {
                continue;
            }
            let curr = adj_mat[i][excluded];
            if curr == ret.0 {
                continue;
            }
            if curr < ret.1 {
                ret.1 = curr;
            }
        }
        ret
    }
    fn kruskal(adj_mat: &Vec<Vec<f64>>, excluded: usize) -> f64 {
        let n: usize = adj_mat.len() - 1;
        let mut disjoint_set: Vec<SetMember> = Vec::new();
        let mut curr = 0;
        let mut ret: f64 = 0f64;
        disjoint_set.resize_with(n, || {
            curr += 1;
            SetMember {
                parent: curr - 1,
                size: 0,
            }
        });
        let closest: (f64, f64) = find_closest(adj_mat, excluded);
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for i in 0..n {
            for j in 0..n {
                if i == j || i == excluded || j == excluded {
                    continue;
                }
                edges.push((i, j));
            }
        }
        edges.sort_by(|x, y| adj_mat[x.0][x.1].partial_cmp(&adj_mat[y.0][y.1]).unwrap()); //O(ELgE)
        for edge in edges {
            let (x, y) = edge;
            if union(&mut disjoint_set, x, y) {
                ret += adj_mat[x][y];
            }
        }
        ret += closest.0 + closest.1;
        ret
    }
    pub fn lower_bound(adj_mat: &Vec<Vec<f64>>) -> f64 {
        let n = adj_mat.len();
        let mut max: f64 = 0f64;
        for i in 0..n {
            let curr = kruskal(&adj_mat, i);
            if curr > max {
                max = curr;
            }
        }
        max
    }
}
fn main() {
    const OUT_PATH: &str = "points.json";
    let mut points: Vec<(u32, u32)> = Vec::new();
    let mut rng = rand::rng();
    const N: u32 = 23;
    const GRID_SIZE: u32 = N * 4;
    for _ in 0..N {
        let mut point: (u32, u32);
        loop {
            point = (
                rng.random_range(0..GRID_SIZE),
                rng.random_range(0..GRID_SIZE),
            );
            if point.0 != point.1 || points.contains(&point) {
                break;
            }
        }
        points.push(point);
    }
    let mut adj_mat: Vec<Vec<f64>> = Vec::new();
    adj_mat.reserve(N as usize);
    for i in 0..N as usize {
        adj_mat.push(Vec::new());
        adj_mat[i].resize(N as usize, 0f64);
        for j in 0..N as usize {
            adj_mat[i][j] = d(points[i], points[j]);
        }
    }
    let lower_bound = kruskal::lower_bound(&adj_mat);
    let mut json_hashmap: HashMap<String, Vec<Vec<f64>>> = HashMap::new();
    let out = File::create(OUT_PATH).expect("Couldn't create output file {OUT_PATH}");
    json_hashmap.insert(
        format!("parameters"),
        vec![vec![f64::from(N)], vec![lower_bound]],
    );
    json_hashmap.insert(format!("adj_mat"), adj_mat);
    serde_json::to_writer(BufWriter::new(out), &json_hashmap).expect("Couldn't write json.");
}
