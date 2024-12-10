fn main() {
    println!("{}", generate_uuid());
}

fn generate_uuid() -> usize {
    // Utc::now().timestamp()
    Utc::now().timestamp_millis().try_into().unwrap()
}