use illu_logi::{gen, vis_grid};
fn main() {
    vis();
}

fn test() {
    let mut sum = std::time::Duration::new(0, 0);
    let n = 20;
    for seed in 0..100 {
        let start = std::time::Instant::now();
        gen(n, n, seed);
        let duration = start.elapsed();
        sum += duration;
        println!("seed = {} : {:?}", seed, duration);
    }
    println!("average : {:?}", sum / 100);
}
fn vis() {
    proconio::input! {
        n:usize,
        seed:u64,
    }
    let board = gen(n, n, seed);
    let svg = vis_grid(n, n, 24, board);
    let vis = format!("<html><body>{}</body></html>", svg);
    std::fs::write("vis.html", &vis).unwrap();
}
