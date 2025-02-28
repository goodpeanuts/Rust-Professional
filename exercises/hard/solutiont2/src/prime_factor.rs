use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, ToPrimitive};
use rand::Rng;

fn is_prime(n: &BigUint) -> bool {
    if n <= &BigUint::one() {
        return false;
    }
    if n <= &BigUint::from(3u32) {
        return true;
    }
    if n.is_even() {
        return false;
    }

    let mut d = n - BigUint::one();
    let mut s = 0;
    while d.is_even() {
        d /= BigUint::from(2u32);
        s += 1;
    }

    let bases: [u32; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    for base in &bases {
        let a = BigUint::from(*base);
        if a >= *n {
            continue;
        }

        let mut x = a.modpow(&d, n);
        if x == BigUint::one() || x == n - BigUint::one() {
            continue;
        }

        let mut composite = true;
        for _ in 0..s - 1 {
            x = x.modpow(&BigUint::from(2u32), n);
            if x == n - BigUint::one() {
                composite = false;
                break;
            }
        }
        if composite {
            return false;
        }
    }
    true
}

fn pollards_rho(n: &BigUint) -> BigUint {
    if n.is_even() {
        return BigUint::from(2u32);
    }

    let mut rng = rand::thread_rng();
    let one = BigUint::one();

    for _ in 0..128 {
        // 使用u32范围生成随机数
        let c: u32 = rng.gen_range(1..1000);
        let x_init: u32 = rng.gen_range(1..1000);

        let c = BigUint::from(c);
        let mut x = BigUint::from(x_init);
        let mut y = x.clone();
        #[allow(unused_assignments)]
        let mut d = BigUint::one();

        for _ in 0..(1 << 18) {
            x = (&x * &x + &c) % n;
            y = (&y * &y + &c) % n;
            y = (&y * &y + &c) % n;
            let diff = if x > y { &x - &y } else { &y - &x };
            d = diff.gcd(n);

            if d != one && &d != n {
                return d;
            }
        }
    }
    one
}

pub fn find_max_prime_factor(n: u128) -> u128 {
    let n = BigUint::from_u128(n).unwrap();
    if n <= BigUint::one() {
        return n.to_u128().unwrap();
    }

    let mut max_prime = BigUint::one();
    let mut stack = vec![n];

    while let Some(current) = stack.pop() {
        if current <= max_prime {
            continue;
        }

        if current < BigUint::from(1_000_000u64) {
            let mut current = current;
            let mut i = BigUint::from(2u32);
            while &i * &i <= current {
                while current.is_multiple_of(&i) {
                    max_prime = std::cmp::max(max_prime.clone(), i.clone());
                    current /= &i;
                }
                i += 1u32;
            }
            if current > BigUint::one() {
                max_prime = std::cmp::max(max_prime, current);
            }
            continue;
        }

        if is_prime(&current) {
            max_prime = std::cmp::max(max_prime, current);
            continue;
        }

        let d = pollards_rho(&current);
        if d == BigUint::one() || d == current {
            max_prime = std::cmp::max(max_prime, current);
        } else {
            stack.push(d.clone());
            stack.push(current / d);
        }
    }

    max_prime.to_u128().unwrap()
}
