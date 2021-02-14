use num::{BigInt, Integer, Unsigned};

pub fn is_prime_6kp1<'a, N>(n: &N) -> bool
where
    N: Integer + Unsigned + Clone,
{
    let zero = N::zero();
    let one = N::one();
    let two = one.clone() + one.clone();
    let three = two.clone() + one.clone();
    let five = three.clone() + two.clone();
    if n == &two || n == &three {
        true
    } else if n <= &one
        || &(n.clone() % two.clone()) == &zero
        || &(n.clone() % three.clone()) == &zero
    {
        false
    } else {
        let mut i = five.clone();

        while &num::pow(i.clone(), 2) <= &n {
            if &(n.clone() % i.clone()) == &zero
                || &(n.clone() % (i.clone() + two.clone())) == &zero
            {
                return false;
            }
            i = i + five.clone();
        }

        true
    }
}

/// Наибольший общий делитель, используя алгоритм евклида.
pub fn gcd_euclid<N: Unsigned + Clone>(mut a: N, mut b: N) -> N {
    while &b != &N::zero() {
        a = a.rem(b.clone());
        std::mem::swap(&mut a, &mut b);
    }
    a
}

/// Возвращаемое значение `x` будет обратным числом для
/// `a` по модулю `b`. Если поменять местами то будет наоборот,
/// `y` - обратное число
pub fn gcd_euclid_extended(a: &BigInt, b: &BigInt) -> ExtendedEuclid<BigInt> {
    if a == &BigInt::from(0 as u32) {
        ExtendedEuclid {
            gcd: b.clone(),
            x: BigInt::from(0 as i32),
            y: BigInt::from(1 as i32),
        }
    } else {
        let b_mod_a = b % a;
        let ExtendedEuclid { gcd: g, x, y } = gcd_euclid_extended(&b_mod_a, &a);
        let b_div_a = b / a;
        ExtendedEuclid {
            gcd: g,
            x: (y - b_div_a * x.clone()),
            y: x,
        }
    }
}

#[derive(Debug)]
pub struct ExtendedEuclid<T> {
    pub gcd: T,
    pub x: T,
    pub y: T,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn are_primes() {
        let known_primes = [
            2_u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
        ];
        for prime in known_primes.iter() {
            assert!(is_prime_6kp1(prime));
        }
        let not_primes = (1_u32..71).filter(|x| known_primes.binary_search(x).is_err());
        for not_a_prime in not_primes {
            assert_eq!(is_prime_6kp1(&not_a_prime), false);
        }
    }
    #[test]
    fn egcd_test() {
        use num::bigint::ToBigInt;
        use std::str::FromStr;

        // small primes
        let a = 179425357u32.to_bigint().unwrap();
        let b = 97u32.to_bigint().unwrap();
        let ExtendedEuclid { gcd: g, x, y } = gcd_euclid_extended(&a, &b);
        assert_eq!(a.clone() * x + b.clone() * y, g);

        // small primes
        let a = 1024u32.to_bigint().unwrap();
        let b = 512u32.to_bigint().unwrap();
        let ExtendedEuclid { gcd: g, x: _, y: _ } = gcd_euclid_extended(&a, &b);
        assert_eq!(512u32.to_bigint().unwrap(), g);

        // big primes
        let known_prime_str = "118595363679537468261258276757550704318651155601593299292198496313960907653004730006758459999825003212944725610469590674020124506249770566394260832237809252494505683255861199449482385196474342481641301503121142740933186279111209376061535491003888763334916103110474472949854230628809878558752830476310536476569";
        let known_prime_str_2 = "357111317192329313741434753596167717379838997101103107109113127131137139149151157163167173179181191193197199211223227229233239241251257263269271277281283293307311313317331337347349353359367373379383389397401409419421431433439443449457461463467479487491499503509521523541547557563569571577587593599601607613617619631641643647653659661673677683691701709719727733739743751757761769773787797809811821823827829839853857859863877881883887907911919929937941947953967971977983991997";
        let a: BigInt = FromStr::from_str(known_prime_str).unwrap();
        let b: BigInt = FromStr::from_str(known_prime_str_2).unwrap();
        let ExtendedEuclid { gcd: g, x, y } = gcd_euclid_extended(&a, &b);
        assert_eq!(a.clone() * x + b.clone() * y, g);
    }
}
