#[derive(Debug)]
struct Bucket {
    liters: u32
}

fn pour(source: &mut Bucket, target: &mut Bucket, amount: u32) {
    source.liters -= amount;
    target.liters += amount;
}

fn main() {
    let mut source_bucket = Bucket { liters: 15 };
    let mut target_bucket = Bucket { liters: 5 };

    pour(&mut source_bucket, &mut target_bucket, 5);

    println!("Source bucket: {:?}", source_bucket);
    println!("Target bucket: {:?}", target_bucket);
}