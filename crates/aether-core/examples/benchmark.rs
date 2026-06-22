use aether_core::benchmarks::run_all_benchmarks;

fn main() {
    println!("========================================");
    println!("  Aether Editor 性能基准测试");
    println!("========================================");
    println!();

    let results = run_all_benchmarks();

    println!();
    println!("========================================");
    println!("  性能测试完成");
    println!("========================================");
    println!();
    println!("共运行 {} 项测试", results.len());
}
