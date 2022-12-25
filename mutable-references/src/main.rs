#[derive(Debug)]
struct Bucket {
    liters: u32
}

fn pour(source: &Bucket, target: &Bucket, amount: u32) {
    source.liters -= amount;
    target.liters += amount;
}

fn main() {
    let source_bucket = Bucket { liters: 15 };
    let target_bucket = Bucket { liters: 5 };

    pour(&source_bucket, &target_bucket, 5);

    println!("Source bucket: {:?}", source_bucket);
    println!("Target bucket: {:?}", target_bucket);
}