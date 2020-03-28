use cargo_snippet::snippet;
#[snippet("mod")]
#[allow(dead_code)]
///refer to https://github.com/hatoo/competitive-rust-snippets
pub const MOD: u64 = 1_000_000_007;

#[snippet("mod")]
#[allow(dead_code)]
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[snippet("mod")]
#[allow(dead_code)]
pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

#[snippet("mod")]
#[allow(dead_code)]
pub fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x, y) = extgcd(b, a % b);
        (gcd, y, x - (a / b) * y)
    }
}

#[snippet("prime")]
#[allow(dead_code)]
pub fn prime(n: u64) -> Vec<bool> {
    let mut prime: Vec<bool> = vec![true; (n+1) as usize];
    prime[0] = false;
    if n >= 1 { prime[1] = false;}
    let mut i = 2_usize;
    while i * i <= n as usize {
        if prime[i] {
            let mut j = i + i;
            while j <= n as usize {
                prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    prime
}

#[snippet("divisor")]
#[allow(dead_code)]
pub fn divisor(n: u64) -> Vec<u64> {
    let mut div: Vec<u64> = Vec::new();
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            div.push(i);
            if i * i != n {
                div.push(n / i);
            }
        }
        i += 1;
    }
    div.sort();
    div
}

use std::collections::HashMap;
#[snippet("prime_factor")]
#[allow(dead_code)]
pub fn prime_factor(n: u64) -> HashMap<u64, u64> {
    let mut prime_factors: HashMap<u64, u64> = HashMap::new();
    let mut n = n;
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            *prime_factors.entry(i).or_insert(0) += 1;
            n /= i;
        }
        i += 1;
    }
    if n != 1 {
        prime_factors.insert(n, 1);
    }
    prime_factors
}

#[snippet("mod")]
#[allow(dead_code)]
pub fn mod_pow(x: u64, n: u64, m: u64) -> u64 {
    let mut res = 1;
    let mut x = x % m;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = (res * x) % m;
        }
        x = (x * x) % m;
        n >>= 1;
    }
    res
}

#[snippet("mod")]
#[allow(dead_code)]
pub fn mod_inv(a: u64, m: u64) -> u64 {
    let (_, x, _) = extgcd(a as i64, m as i64);
    ((m as i64 + x) as u64 % m) % m
}

#[snippet("mod")]
#[allow(dead_code)]
pub fn fact_table(size: usize, m: u64) -> Vec<u64> {
    let mut table = vec![1; size + 1];
    for i in 1..size + 1 {
        table[i] = (i as u64 * table[i - 1]) % m;
    }
    table
}

#[snippet("mod")]
#[allow(dead_code)]
/// Factorial and Inverse factorial table
pub fn fact_inv_table(size: usize, m: u64) -> (Vec<u64>, Vec<u64>) {
    let mut fact = vec![1; size];
    let mut fact_inv = vec![1; size];

    for i in 2..size {
        fact[i] = fact[i - 1] * i as u64 % m;
        fact_inv[i] = m - ((m / i as u64) * fact_inv[(m % i as u64) as usize] % m);
    }

    for i in 1..size {
        fact_inv[i] = fact_inv[i - 1] * fact_inv[i] % m;
    }

    (fact, fact_inv)
}

#[snippet("mod")]
#[allow(dead_code)]
/// (a mod p, e when n! = a pe)
pub fn mod_fact(n: u64, p: u64, fact: &[u64]) -> (u64, u64) {
    if n == 0 {
        (1, 0)
    } else {
        let (a, b) = mod_fact(n / p, p, fact);
        let pow = b + n / p;

        if n / p % 2 != 0 {
            (a * (p - fact[(n % p) as usize]) % p, pow)
        } else {
            (a * fact[(n % p) as usize] % p, pow)
        }
    }
}

#[snippet("mod")]
#[allow(dead_code)]
/// C(n, k) % p
pub fn mod_comb(n: u64, k: u64, p: u64, fact: &[u64]) -> u64 {
    if n < k {
        0
    } else {
        let (a1, e1) = mod_fact(n, p, fact);
        let (a2, e2) = mod_fact(k, p, fact);
        let (a3, e3) = mod_fact(n - k, p, fact);

        if e1 > e2 + e3 {
            0
        } else {
            a1 * mod_inv(a2 * a3 % p, p) % p
        }
    }
}

#[snippet("mod")]
#[allow(dead_code)]
/// H(n, k) % p
pub fn mod_comb_repetition(n: u64, k: u64, p: u64, fact: &[u64]) -> u64 {
    mod_comb(n - 1 + k, n - 1, p, fact)
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(3, 5), 15);
}

#[test]
fn test_prime() {
    assert_eq!(vec![false, false, true, true, false], prime(4));
    assert_eq!(vec![false, false, true, true, false, true, false, true, false, false, false], prime(10));
}

#[test]
fn test_divisor() {
    assert_eq!(divisor(10), vec![1, 2, 5, 10]);
}

#[test]
fn test_prime_factor() {
    let mut hoge: HashMap<u64, u64> = HashMap::new();
    hoge.insert(2, 3);
    assert_eq!(prime_factor(8), hoge);
    hoge.clear();
    hoge.insert(1_000_000_007, 1);
    assert_eq!(prime_factor(1_000_000_007), hoge);
}

#[test]
fn test_mod_pow() {
    assert_eq!(mod_pow(2, 3, MOD), 8);
    assert_eq!(mod_pow(2, 3, 6), 2);
    assert_eq!(mod_pow(2, 10, MOD), 1024);
}

#[test]
fn test_fact_inv_table() {
    let m = 1_000_000_007;
    let size = 1000;
    let (fact, fact_inv) = fact_inv_table(1000, m);

    for i in 0..size {
        assert_eq!(fact[i] * fact_inv[i] % m, 1);
    }
}

#[test]
fn test_mod_comb_repetition() {
    let m = 1_000_000_007;
    let fact = fact_table(200000, m);

    assert_eq!(mod_comb_repetition(10, 2, m, &fact), 55);
    assert_eq!(mod_comb_repetition(10, 3, m, &fact), 220);
    assert_eq!(mod_comb_repetition(10, 4, m, &fact), 715);
    assert_eq!(mod_comb_repetition(400, 296, m, &fact), 546898535);
    assert_eq!(mod_comb_repetition(100000, 100000, m, &fact), 939733670);
}