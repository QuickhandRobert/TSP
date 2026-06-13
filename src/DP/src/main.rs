use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::ops::Div;
fn held_karp(adj_mat: &Vec<Vec<f64>>) -> (f64, Vec<u32>) {
    let n: u32 = adj_mat.len() as u32;
    let mut c: HashMap<(u32, u32), (f64, u32)> = HashMap::new();
    for i in 1..n {
        c.insert((1 << i, i), (adj_mat[0][i as usize], 0));
    }
    let bar = ProgressBar::new(2u64.pow(n) * n.pow(2).div(10) as u64 + n as u64 * 2);
    bar.set_style(ProgressStyle::with_template("{percent}% [{elapsed_precise}]").unwrap());
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
                let mut res: Vec<(f64, u32)> = Vec::new();
                for m in subset.clone() {
                    bar.inc(1);
                    if *m == 0 || *m == *k {
                        continue;
                    }
                    res.push((
                        c.get(&(prev, *m)).unwrap().0 + adj_mat[*m as usize][*k as usize],
                        *m,
                    ));
                    let min_val: &(f64, u32) = res
                        .iter()
                        .min_by(|x, y| x.0.partial_cmp(&y.0).unwrap())
                        .unwrap();
                    c.insert((bits, *k), *min_val);
                }
            }
        }
    }
    let mut bits: u32 = (2u32.pow(n) - 1) - 1;
    let mut res: Vec<(f64, u32)> = Vec::new();
    for k in 1..n {
        bar.inc(1);
        res.push((c.get(&(bits, k)).unwrap().0 + adj_mat[k as usize][0], k));
    }
    let (opt, mut parent): (f64, u32) = *res
        .iter()
        .min_by(|x, y| x.0.partial_cmp(&y.0).unwrap())
        .unwrap();
    let mut path: Vec<u32> = Vec::new();
    for _ in 1..(n - 1) {
        bar.inc(1);
        path.push(parent);
        let new_bits: u32 = bits & !(1 << parent);
        parent = c.get(&(bits, parent)).unwrap().1;
        bits = new_bits;
    }
    bar.finish();
    (opt, path)
}
fn main() {
    const IN_PATH: &str = "adj_mat.json";
    let input_file = File::open(IN_PATH).expect("Couldn't open input file.");
    println!("Loading json file...");
    let v: HashMap<String, Vec<Vec<f64>>> =
        serde_json::from_reader(BufReader::new(input_file)).expect("Couldn't parse json file");
    let parameters = v
        .get("parameters")
        .expect("Invalid json! couldn't find \"parameters\"");
    let (_, approx) = (parameters[0][0], parameters[1][0]);
    let adj_mat = v
        .get("adj_mat")
        .expect("Invalid json! couldn't find \"adj_mat\"");
    println!("Json Loaded Succesfully!");
    println!("Running the Held Karp algorithm...");
    let (cost, path) = held_karp(adj_mat);
    println!("Route Found!\nCost: {cost}\nPath: {path:?}");
    println!(
        "[Benchmark]\nRatio to Lower Bound: {ratio}",
        ratio = cost / approx
    );
}
