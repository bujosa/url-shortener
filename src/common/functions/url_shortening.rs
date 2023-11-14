use sha2::{Digest, Sha256};

pub fn hash_url_for_shortening(url: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)[0..7].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_url_for_shortening() {
        let url = "https://www.rust-lang.org/";
        let hashed_url = hash_url_for_shortening(url);
        assert_eq!(hashed_url, "0a6e6cc");
    }
}
