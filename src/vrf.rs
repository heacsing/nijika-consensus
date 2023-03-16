use bincode;
use serde::Serialize;
use vrf::openssl::{CipherSuite, ECVRF, Error};
use vrf::VRF;
use rand::prelude::Rng;
use rand_chacha::{ChaCha8Rng, rand_core::SeedableRng};
use crate::primitives::NijikaNodeRole;

pub struct NijikaVRFClientS {
    client: ECVRF,
}

#[derive(Serialize, Debug)]
pub struct NijikaVRFParams {
    pub weight: u64,
    pub round: u64,
    pub seed: u64,
    pub role: NijikaNodeRole,
}

impl NijikaVRFClientS {
    pub fn new() -> Self {
        let vrf = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();
        NijikaVRFClientS { client: vrf }
    }
    pub fn gen_keys(&mut self, seed: u64) -> Result<(Vec<u8>, Vec<u8>), Error> {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let size: u64 = rng.gen_range(10..256);
        let mut secret_key = vec![0 as u8; size as usize];
        for i in secret_key.iter_mut() {
            *i = rng.gen();
        }
        match self.client.derive_public_key(&secret_key) {
            Ok(public_key) => {
                return Ok((secret_key, public_key));
            },
            Err(e) => Err(e)
        }
    }
    pub fn prove(&mut self, secret_key: &[u8], data: &NijikaVRFParams) -> Result<(Vec<u8>, Vec<u8>), Error> {
        let nijika_vrf_params = bincode::serialize(data).unwrap();
        match self.client.prove(secret_key, &nijika_vrf_params) {
            Ok(proof) => {
                match self.client.proof_to_hash(&proof) {
                    Ok(hash) => {
                        Ok((proof, hash))
                    },
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }
    }
    pub fn verify(&mut self, public_key: &[u8], proof: &[u8], data: &NijikaVRFParams, hash: &[u8]) -> Result<bool, Error> {
        let nijika_vrf_params = bincode::serialize(data).unwrap();
        match self.client.verify(public_key, proof, &nijika_vrf_params) {
            Ok(beta) => {
                if beta == hash {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            },
            Err(e) => Err(e)
        }
    }
}

struct bernoulli {
    pub selected_unit: u64,
    own_units: u64,
    probability: f64,
}

pub fn calculate_prospect(own_units:u64, expect_units: u64, total_unit: u64) {
    let prospect: f64 = expect_units as f64 / total_unit as f64;
    let mut index: u64 = 0;
}


mod tests {
    use super::{NijikaNodeRole, NijikaVRFClientS, NijikaVRFParams};
    #[test]
    fn work() {
        let mut vrf = NijikaVRFClientS::new();
        let (s, p) = vrf.gen_keys(5).unwrap();
        let data = NijikaVRFParams {
            weight: 10,
            round: 12,
            seed: 128,
            role: NijikaNodeRole::NORMAL
        };
        let (proof, hash) = vrf.prove(&s, &data).unwrap();
        println!("generating proof and hash: {:#?} \r {:#?}", &proof, &hash);
        println!("hash len: {}", hash.len());
        let result = vrf.verify(&p, &proof, &data, &hash);
        match result {
            Ok(flag) => assert!(flag),
            Err(_) => {
                println!("verify failed");
            }
        }
    }
}
