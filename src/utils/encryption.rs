use base64ct::{Base64, Encoding};
use sha2::{Digest, Sha512};

const BUF_SIZE: usize = 128;

pub async fn hash_password(password: String) -> Option<String> {
    let mut hasher = Sha512::new();
    hasher.update(password.as_bytes());
    let hash = hasher.finalize();
    let mut enc_buf = [0u8; BUF_SIZE];
    let result = Base64::encode(&hash, &mut enc_buf);
    match result {
        Ok(data) => Some(String::from(data)),
        Err(_err) => None
    }
}
