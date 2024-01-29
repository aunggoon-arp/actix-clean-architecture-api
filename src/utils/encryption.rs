use base64ct::{Base64, Encoding};
use sha2::{Digest, Sha512};

pub async fn hash_password(password: String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(password.as_bytes());
    let hash = hasher.finalize();
    Base64::encode_string(&hash)
}
