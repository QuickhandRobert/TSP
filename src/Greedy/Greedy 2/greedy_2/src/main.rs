use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
fn farthest_insertion(adj_mat: &Vec<Vec<f64>>) -> (f64, Vec<u32>) {
    let n = adj_mat.len();
    let mut curr = 0;
    let mut v: Vec<usize> = Vec::new();
    v.resize_with(n - 1, || {
        curr += 1;
        curr
    });
    let mut parents: Vec<usize> = Vec::new();
    parents.resize(n, 0);
    let mut t: Vec<usize> = Vec::new();
    t.push(0);
    while !v.is_empty() {
        let mut min: f64 = f64::INFINITY;
        let mut k = 0;
        let mut i_fin: usize = 0;
        let mut j_fin: usize = 0;
        for j in t.iter() {
            let mut max: f64 = 0f64;
            for to in v.iter() {
                if adj_mat[*j][*to] > max {
                    max = adj_mat[*j][*to];
                    k = *to;
                }
            }
            let i = parents[*j]; //Parent
            let dist = adj_mat[i][k] + adj_mat[k][*j] - adj_mat[i][*j];
            if dist < min {
                min = dist;
                (i_fin, j_fin) = (i, *j);
            }
        }
        v = v.into_iter().filter(|x| *x != k).collect();
        t.push(k);
        if j_fin != 0 {
            parents[j_fin] = k;
        }
        parents[k] = i_fin;
    }
    let mut path: Vec<u32> = Vec::new();
    let mut i = 1;
    let mut cost: f64 = 0f64;
    loop {
        path.push(i as u32);
        if parents[i] == 0 {
            break;
        }
        cost += adj_mat[i][parents[i]];
        i = parents[i];
    }
    (cost, path)
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
    println!("Running the Farthest-Insertion algotirhm...");
    let (cost, path) = farthest_insertion(adj_mat);
    println!("Route Found!\nCost: {cost}\nPath: {path:?}");
    println!(
        "[Benchmark]\nRatio to Lower Bound: {ratio}",
        ratio = cost / approx
    );
}
