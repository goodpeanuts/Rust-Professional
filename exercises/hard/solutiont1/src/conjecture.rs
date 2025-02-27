use std::{
    sync::{LazyLock, Mutex},
    vec,
};

static PRIMEAS: LazyLock<Mutex<Vec<bool>>> = LazyLock::new(|| {
    let mut v = vec![true; 10240];
    v[0] = false;
    v[1] = false;
    Mutex::new(v)
});

static THRESHOLD: LazyLock<Mutex<usize>> = LazyLock::new(|| Mutex::new(2));
static TARGET: LazyLock<Mutex<usize>> = LazyLock::new(|| Mutex::new(3));

static SQUARE: LazyLock<Mutex<Vec<usize>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static SQRT: LazyLock<Mutex<usize>> = LazyLock::new(|| Mutex::new(1));

pub fn goldbach_conjecture() -> String {
    sieve_of_eratosthenes();

    let mut res = Vec::new();
    let primes = PRIMEAS.lock().expect("get PRIMES failed");
    let len = primes.len();
    let mut target = *TARGET.lock().expect("get threshold failed");
    let square = SQUARE.lock().expect("get square failed");
    while target < len {
        if !primes[target] {
            let mut flag = true;
            for (i, is_prime) in primes.iter().enumerate() {
                if i >= target {
                    break;
                }
                let rem = target - i;
                let sq = if !*is_prime || rem % 2 == 1 {
                    continue;
                } else {
                    rem / 2
                };
                if square.contains(&sq) {
                    flag = false;
                    break;
                }
            }
            if flag {
                res.push(target);
                if res.len() == 2 {
                    break;
                }
            }
        }
        target += 2;
    }
    res.iter()
        .map(|t| t.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn sieve_of_eratosthenes() {
    let mut primes = PRIMEAS.lock().expect("get PRIMES failed");
    let mut threshold = THRESHOLD.lock().expect("get threshold failed");

    let len = primes.len() * 2;
    primes.resize(len, true);
    let sqrt_max = (len as f64).sqrt() as usize;

    for i in *threshold..=sqrt_max {
        if primes[i] {
            for j in (i * i..len).step_by(i) {
                primes[j] = false;
            }
        }
    }

    for (idx, is_prime) in primes.iter().rev().enumerate() {
        if *is_prime {
            *threshold = primes.len() - 1 - idx;
            break;
        }
    }

    let mut square = SQUARE.lock().expect("get Square failed");
    let mut sqrt = *SQRT.lock().expect("get sqrt failed");
    while sqrt * sqrt <= len {
        square.push(sqrt * sqrt);
        sqrt += 1;
    }
}
