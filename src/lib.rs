use algorithms::{gcd_euclid, gcd_euclid_extended, is_prime_6kp1};
use colored::Colorize;
use num::{
    bigint::{Sign, ToBigInt},
    iter::Range,
    pow::Pow,
    BigInt, BigUint, Signed,
};
use rand::prelude::SliceRandom;
use std::fmt::Display;

pub mod algorithms;

#[derive(Debug)]
pub struct RsaPublicKey {
    pub n: BigUint,
    pub e: BigUint,
}

impl RsaPublicKey {
    pub fn new_from_p_q(p: &BigUint, q: &BigUint) -> Self {
        assert!(is_prime_6kp1(p), "`p` должно быть простым числом");
        assert!(is_prime_6kp1(q), "`q` должно быть простым числом");
        let mut rng = rand::thread_rng();
        let n = p * q; // первая часть открытого ключа

        let fi = {
            let one = BigUint::from(1_u32);
            (p - &one) * (q - &one)
        };

        // вторая часть открытого ключа
        let e = {
            let range: Range<BigUint> = num::range(3_u32.into(), fi.clone() - BigUint::from(1_u32));
            let values = range
                // .filter(|x| x.gcd(&fi) == 1_u32.into())
                .filter(|x| gcd_euclid(x.clone(), fi.clone()) == 1_u32.into())
                .collect::<Vec<_>>();

            values
                .choose(&mut rng)
                .expect("ошибка: не найдено ниодного допустимого значения для `e`")
                .clone()
        };
        RsaPublicKey { e, n }
    }
    pub fn new_from_n_e(n: BigUint, e: BigUint) -> Self {
        RsaPublicKey { e, n }
    }
    /// `c = m ^ e % n`
    pub fn encrypt(&self, msg: &BigUint) -> BigUint {
        Pow::pow(msg, &self.e) % &self.n
    }
    /// `m' = s ^ e % n`
    pub fn check_sign(&self, msg: &BigUint, sign: &BigUint) -> bool {
        let mm = Pow::pow(sign, &self.e) % &self.n;
        &mm == msg
    }
}
impl Display for RsaPublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "n = {}, e = {}",
            self.n.to_str_radix(10).green(),
            self.e.to_str_radix(10).bright_green()
        )
    }
}

#[derive(Debug)]
pub struct RsaPrivateKey {
    pub n: BigUint,
    pub d: BigUint,
}

impl RsaPrivateKey {
    pub fn new_from_n_d(n: BigUint, d: BigUint) -> Self {
        Self { n, d }
    }
    pub fn new_from_e_p_q(e: &BigUint, p: &BigUint, q: &BigUint) -> Self {
        assert!(is_prime_6kp1(p), "`p` должно быть простым числом");
        assert!(is_prime_6kp1(q), "`q` должно быть простым числом");
        let fi = {
            let one = BigUint::from(1_u32);
            (p - &one) * (q - &one)
        };
        let n = p * q; // первая часть открытого ключа
        let e = e.to_bigint().unwrap();
        let fi1 = fi.clone().to_bigint().unwrap();
        let mut d = gcd_euclid_extended(&e, &fi1).x;

        while d.is_negative() {
            // Modular inverse.
            d = d + BigInt::from_biguint(Sign::Plus, fi.clone());
        }
        let d = d.to_biguint().unwrap();
        Self { n, d }
    }
    /// `m' = c ^ d % n`
    pub fn decrypt(&self, cipher: &BigUint) -> BigUint {
        let c_pow_d = Pow::pow(cipher, &self.d);
        c_pow_d % &self.n
    }

    /// `s = m ^ d % n`
    pub fn sign(&self, message: &BigUint) -> BigUint {
        Pow::pow(message, &self.d) % &self.n
    }
}

impl Display for RsaPrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "n = {}, d = {}",
            self.n.to_str_radix(10).green(),
            self.d.to_str_radix(10).magenta()
        )
    }
}

pub struct KeyPair {
    pub public: RsaPublicKey,
    pub private: RsaPrivateKey,
}

impl KeyPair {
    pub fn new_from_p_q(p: &BigUint, q: &BigUint) -> Self {
        let public = RsaPublicKey::new_from_p_q(p, q);
        let private = RsaPrivateKey::new_from_e_p_q(&public.e, p, q);
        Self { public, private }
    }
}
