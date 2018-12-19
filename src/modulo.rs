#[snippet = "mod"]
#[allow(dead_code)]
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[snippet = "mod"]
#[allow(dead_code)]
pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

#[snippet = "mod"]
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
    loop {
        if i * i > n { break; }
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