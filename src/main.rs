use sha2::{Digest, Sha256};

fn main() {
    let url = "https://github.com/bujosa";
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    let result = hasher.finalize();
    let short_id = hex::encode(result)[0..7].to_string();
    println!("Generated short ID: {}", short_id);
}
