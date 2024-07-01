use crate::models::HashedPassword;
use password_hash::SaltString;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use sha2::{Digest, Sha256};

const SALT_LENGTH: usize = 16;

pub fn hash(password: &str, salt: &str) -> String {
    let salted_string = format!("{}:{}", password, salt);
    //println!("{}", salted_string);
    let mut hasher = Sha256::new();

    hasher.update(salted_string);

    let result = hasher.finalize();

    let res = hex::encode(result);

    return res;
}

pub fn hash_password(password: &str) -> HashedPassword {
    let salt = salt_generator();
    let salted_string = format!("{}:{}", password, salt);
    //println!("{}", salted_string);
    let mut hasher = Sha256::new();

    hasher.update(salted_string);

    let result = hasher.finalize();

    let hash = hex::encode(result);

    return HashedPassword {
        hashed_password: hash,
        salt,
    };
}

fn salt_generator() -> String {
    let mut bytes: [u8; SALT_LENGTH] = [0; SALT_LENGTH];
    StdRng::from_entropy().fill_bytes(&mut bytes);
    return String::from(SaltString::encode_b64(&bytes).unwrap().as_str());
}

pub fn check_password(password: &str, salt: &str, password_hash: &str) -> bool {
    return password_hash == hash(password, salt);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let hashed = hash("string", "string");
        assert_eq!(
            hashed,
            "5e2b66ce4f42f50784824384c3bfe6872c21c08a9cbfb1397e6e34d7fac53997"
        );
    }
    #[test]
    fn can_check_password() {
        let check = check_password(
            "string",
            "string",
            "5e2b66ce4f42f50784824384c3bfe6872c21c08a9cbfb1397e6e34d7fac53997",
        );
        assert_eq!(check, true);
    }
    #[test]
    fn can_find_wrong_password() {
        let check = check_password("string", "string", "2wtewew");
        assert_eq!(check, false);
    }
}
