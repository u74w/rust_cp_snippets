#[snippet = "MOD"]
#[allow(dead_code)]
pub const MOD: u64 = 1_000_000_007;

#[snippet = "gcd"]
#[allow(dead_code)]
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[snippet = "lcm"]
#[snippet(include = "gcd")]
#[allow(dead_code)]
pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

#[snippet = "extgcd"]
#[snippet(include = "gcd")]
#[allow(dead_code)]
pub fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x, y) = extgcd(b, a % b);
        (gcd, y, x - (a / b) * y)
    }
}

#[snippet = "prime"]
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

#[snippet = "divisor"]
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
#[snippet = "prime_factor"]
#[allow(dead_code)]
pub fn prime_factor(n: u64) -> HashMap<u64, u64> {
    let mut prime_factors: HashMap<u64, u64> = HashMap::new();
    let mut n = n;
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            if prime_factors.contains_key(&i) {
                let exponent = prime_factors[&i];
                prime_factors.insert(i, exponent + 1);
            } else {
                prime_factors.insert(i, 1);
            }
            n /= i;
        }
        i += 1;
    }
    if n != 1 {
        prime_factors.insert(n, 1);
    }
    prime_factors
}

#[snippet = "mod_pow"]
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