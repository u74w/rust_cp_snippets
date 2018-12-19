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
pub fn prime(n: usize) -> Vec<bool> {
    let mut prime: Vec<bool> = vec![true; (n+1) as usize];
    prime[0] = false;
    if n >= 1 { prime[1] = false;}
    let mut i = 2_usize;
    loop {
        if i * i > n { break; }
        if prime[i] {
            let mut j = i + i;
            loop {
                if j > n { break; }
                prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    return prime;
}

#[snippet = "divisor"]
#[allow(dead_code)]
pub fn divisor(n: u64) -> Vec<u64> {
    let mut div: Vec<u64> = Vec::with_capacity(n as usize);
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
    return div;
}

use std::collections::HashMap;
#[snippet = "prime_factor"]
#[allow(dead_code)]
pub fn prime_factor(mut n: u64) -> HashMap<u64, u64> {
    let mut prime_factors: HashMap<u64, u64> = HashMap::with_capacity(n as usize);
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            if prime_factors.contains_key(&i) {
                prime_factors.insert(i, prime_factors[&i]+1);
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
    return prime_factors;
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
}