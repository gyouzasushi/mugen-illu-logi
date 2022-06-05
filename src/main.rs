use illu_logi::gen;
fn main() {
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
