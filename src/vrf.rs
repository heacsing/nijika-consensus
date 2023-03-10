use serde_json;
use serde::{Serialize};
use vrf::openssl::{CipherSuite, ECVRF, Error};
use vrf::VRF;
use rand::prelude::Rng;
use rand_chacha::{ChaCha8Rng, rand_core::SeedableRng};
use crate::primitives::NijikaNodeType;

pub struct NijikaVRFClientS {
    client: ECVRF,
}

#[derive(Serialize, Debug)]
pub struct NijikaVRFParams<U> {
    weight: U,
    round: u64,
    seed: u64,
    role: NijikaNodeType,
}

impl NijikaVRFClientS {
    fn new() -> Self {
        let vrf = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();
        NijikaVRFClientS { client: vrf }
    }
    fn gen_keys(&mut self, seed: u64) -> Result<(Vec<u8>, Vec<u8>), Error> {
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
    fn prove(&mut self, secret_key: &[u8], data: &NijikaVRFParams<u64>) -> Result<(Vec<u8>, Vec<u8>), Error> {
        let nijika_vrf_params = serde_json::to_string(data).unwrap();
        match self.client.prove(secret_key, nijika_vrf_params.as_bytes()) {
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
    fn verify(&mut self, public_key: &[u8], proof: &[u8], data: &NijikaVRFParams<u64>, hash: &[u8]) -> Result<bool, Error> {
        let nijika_vrf_params = serde_json::to_string(data).unwrap();
        match self.client.verify(public_key, proof, nijika_vrf_params.as_bytes()) {
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



mod tests {
    use super::*;

    #[test]
    fn work() {
        let mut vrf = NijikaVRFClientS::new();
        let (s, p) = vrf.gen_keys(5).unwrap();
        let data = NijikaVRFParams {
            weight: 10,
            round: 12,
            seed: 128,
            role: NijikaNodeType::NORMAL
        };
        let (proof, hash) = vrf.prove(&s, &data).unwrap();
        println!("generating proof and hash: {:#?} \r {:#?}", &proof, &hash);

        let result = vrf.verify(&p, &proof, &data, &hash);
        match result {
            Ok(flag) => assert!(flag),
            Err(_) => {
                println!("verify failed");
            }
        }
    }
}
