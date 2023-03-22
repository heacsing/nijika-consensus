use bincode;
use rug::{Float, Integer};
use rug::ops::Pow;
use serde::Serialize;
use vrf::openssl::{CipherSuite, ECVRF, Error};
use vrf::VRF;
use rand::prelude::Rng;
use rand_chacha::{ChaCha8Rng, rand_core::SeedableRng};
use crate::primitives::NijikaNodeRole;

pub struct NijikaVRFClientS {
    client: ECVRF,
    binomial_bounds: Vec<Float>
}

#[derive(Serialize, Debug)]
pub struct NijikaVRFParams {
    pub weight: u64,
    pub round: u64,
    pub seed: u64,
    pub role: NijikaNodeRole,
}

impl NijikaVRFClientS {
    pub fn new_raw() -> Self {
        let vrf = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();
        NijikaVRFClientS { client: vrf, binomial_bounds: vec![] }
    }
    pub fn new(own_units:u64, expect_units: u64, total_unit: u64) -> Self {
        let vrf = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();
        let /*p*/prospect = Float::with_val(113, expect_units) / Float::with_val(113, total_unit);
        let tmp_res = Float::with_val(113, 0);
        let mut res = vec![tmp_res];
        for index in (0..own_units) {
            let l1 = prospect.clone().pow(index);
            let l2 = (Float::with_val(113, 1)-prospect.clone()).pow(own_units - index);
            let multi = l1 * l2 * Integer::from(own_units).binomial(index.try_into().unwrap());
            res.push(multi + res.last().expect("cannot be a empty vec"));
        }
        Self {client: vrf, binomial_bounds: res}
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
    pub fn sortition(&self, bytes: &[u8]) -> (u64, Float) {
        let divisor = Integer::from(Integer::i_pow_u(2, 256));
        let dividend = Float::with_val(256, Integer::from_digits(bytes, rug::integer::Order::Lsf));
        let val = dividend / divisor;
        let len = self.binomial_bounds.len();
        for i in (1..len) {
            if val < self.binomial_bounds[i] {
                return (i as u64 - 1, val);
            }
        }
        return (len as u64, val);
    }
}

mod tests {
    use std::ops::Div;

    use rug::{Integer, Float};

    use crate::primitives::ByteArray;

    use super::{NijikaNodeRole, NijikaVRFClientS, NijikaVRFParams};
    type my_hash = ByteArray<32>;
    #[test]
    fn work() {
        let mut vrf = NijikaVRFClientS::new(10, 100, 1000);
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
        let a = my_hash::random();
        let (i,val) = vrf.sortition(a.as_bytes());
        println!("gen sortition array: {:#?}", vrf.binomial_bounds);
        println!("gen hash value: {}", val);
        println!("told that is in index: {}", i);
    }
}
