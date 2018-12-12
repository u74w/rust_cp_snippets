#[snippet = "template"]
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufWriter, Write, StdoutLock};
#[snippet = "template"]
#[allow(unused_imports)]
use std::cmp::{max, min, Ordering};
#[snippet = "template"]
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[snippet = "template"]
#[allow(unused_imports)]
use std::iter::FromIterator;

#[snippet = "template"]
#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[snippet = "template"]
#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[snippet = "template"]
#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, bytes) => {
        read_value!($next, String).into_bytes()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[snippet = "template"]
#[allow(dead_code)]
pub fn with_bufwriter<F: FnOnce(BufWriter<StdoutLock>) -> ()>(f: F) {
    let out = stdout();
    let writer = BufWriter::new(out.lock());
    f(writer)
}

#[snippet = "template"]
#[allow(unused_macros)]
macro_rules! debug {
	($($a:expr),*) => {
		eprintln!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*)
	};
}

#[snippet = "template"]
#[allow(dead_code)]
const BIG_STACK_SIZE: bool = true;

#[snippet = "template"]
#[allow(dead_code)]
fn main() {
	use std::thread;
	if BIG_STACK_SIZE {
		thread::Builder::new()
			.stack_size(32 * 1024 * 1024)
			.name("solve".into())
			.spawn(solve)
			.unwrap()
			.join()
			.unwrap();
	} else {
		solve();
	}
}

#[snippet = "template"]
#[allow(dead_code)]
fn solve() {

}

