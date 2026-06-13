use rand::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
fn d((x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> u32 {
    let x_dist = if x2 > x1 { x2 - x1 } else { x1 - x2 };
    let y_dist = if y2 > y1 { y2 - y1 } else { y1 - y2 };
    x_dist ^ 2 + y_dist ^ 2
}
fn held_karp(points: Vec<(u32, u32)>) -> (u32, Vec<u32>) {
    let n: u32 = points.len() as u32;
    let mut c: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
    for i in 1..n {
        c.insert((1 << i, i), (d(points[0], points[i as usize]), 0));
    }
    for subset_size in 2..n {
        let items: Vec<u32> = (1..n).collect();
        for subset in gen_combinations::CombinationIterator::new(&items, subset_size as usize) {
            let mut bits = 0;
            for bit in subset.clone() {
                //Iterating the iterator kinda destroys it, so, clone is a good idea I believe
                bits |= 1 << bit;
            }
            for k in subset.clone() {
                let prev: u32 = bits & !(1 << k);
                let mut res: Vec<(u32, u32)> = Vec::new();
                for m in subset.clone() {
                    if *m == 0 || *m == *k {
                        continue;
                    }
                    res.push((
                        c.get(&(prev, *m)).unwrap().0 + d(points[*m as usize], points[*k as usize]),
                        *m,
                    ));
                    let min_val: &(u32, u32) = res.iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap();
                    c.insert((bits, *k), *min_val);
                }
            }
        }
    }
    println!("{c:?}");
    let mut bits: u32 = (2 ^ n - 1) - 1;
    let mut res: Vec<(u32, u32)> = Vec::new();
    for k in 1..n {
        res.push((
            c.get(&(bits, k)).unwrap().0 + d(points[k as usize], points[0]),
            k,
        ));
    }
    let (opt, mut parent): (u32, u32) = *res.iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap();
    let mut path: Vec<u32> = Vec::new();
    for i in 1..(n - 1) {
        path.push(parent);
        let new_bits: u32 = bits & !(1 << parent);
        parent = c.get(&(bits, parent)).unwrap().1;
        bits = new_bits;
    }
    (opt, path)
}

fn main() {
    const OUT_PATH: &str = "points";
    let mut points: Vec<(u32, u32)> = Vec::new();
    let mut rng = rand::rng();
    const N: u32 = 8;
    const GRID_SIZE: u32 = N * 4;
    points.push((0, 0));
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
    let (opt, path) = held_karp(points);
    println!("{path:?}");
    // let out = File::create(OUT_PATH).expect("Couldn't create output file {OUT_PATH}");
    // let mut writer = BufWriter::new(out);
    // for point in points {
    //     writer
    //         .write_fmt(format_args!("{x} {y}\n", x = point.0, y = point.1))
    //         .expect("Couldn't write into {out}");
    // }
}
