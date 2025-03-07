const MAX_N: usize = 100000;

pub fn goldbach_conjecture() -> String {
    let mut prime: Vec<usize> = Vec::new();
    let mut is_prime: Vec<usize> = vec![0; MAX_N + 5];

    for i in 2..=MAX_N {
        if is_prime[i] == 0 {
            prime.push(i);
        }

        for t in &prime {
            let num = i * t;
            if num > MAX_N {
                break;
            }
            is_prime[num] = 1;
            if i % t == 0 {
                break;
            }
        }
    }

    let mut answer: Vec<usize> = Vec::new();

    for i in (9..=MAX_N).step_by(2) {
        if is_prime[i] == 0 {
            continue;
        }
        if check(i, &prime) {
            continue;
        }
        answer.push(i);
        if answer.len() >= 2 {
            break;
        }
    }

    format!("{},{}", answer[0], answer[1])
}

fn check(n: usize, pr: &Vec<usize>) -> bool {
    for it in pr {
        if *it >= n {
            break;
        }

        if binary_search(1, n - it, n - it) {
            return true;
        }
    }

    false
}

fn binary_search(mut l: usize, mut r: usize, x: usize) -> bool {
    if l > r {
        return false;
    }

    let mid = (l + r) / 2;
    let sqrt_2 = 2 * mid * mid;

    match sqrt_2.cmp(&x) {
        std::cmp::Ordering::Equal => return true,
        std::cmp::Ordering::Less => l = mid + 1,
        std::cmp::Ordering::Greater => r = mid - 1,
    }

    binary_search(l, r, x)
}
