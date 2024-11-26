use sha2::{Digest, Sha512};

const BUF_SIZE: usize = 512;

pub async fn hash_password(password: String) -> Option<String> {
    if password.is_empty() {
        return None;
    }

    let mut hasher = Sha512::new();
    hasher.update(password.as_bytes());
    let hash = hasher.finalize();
    Some(hex::encode(hash))
}
