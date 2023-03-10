use openssl::sha::Sha512;

use crate::primitives::HashValue;

pub struct hash {
    hasher: Sha512
}

impl hash {
    pub fn new(content: &[u8]) -> HashValue {
        let mut hasher = Sha512::new();
        hasher.update(content);
        HashValue::new(hasher.finish())
    }

    pub fn has_target_hash(target: &HashValue, pool: &Vec<HashValue>) -> bool {
        for item in pool {
            if item == target {
                return true;
            }
        }
        return false;
    }
}
