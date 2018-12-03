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
#[allow(dead_code)]
pub fn with_bufwriter<F: FnOnce(BufWriter<StdoutLock>) -> ()>(f: F) {
    let out = stdout();
    let writer = BufWriter::new(out.lock());
    f(writer)
}

#[snippet = "template"]
#[allow(unused_macros)]
macro_rules! input {
	(source = $s:expr, $($r:tt)*) => {
		let mut iter = $s.split_whilespace();
		input_inner!{iter, $($r)*}
	};
	($($r:tt)*) => {
		let mut s = {
			use std::io::Read;
			let mut s = String::new();
			std::io::stdin().read_to_string(&mut s).unwrap();
			s
		};
		let mut iter = s.split_whitespace();
		input_inner!{iter, $($r)*}
	};
}

#[snippet = "template"]
#[allow(unused_macros)]
macro_rules! input_inner {
	($iter:expr) => {};
	($iter:expr, ) => {};

	($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
		let $var = read_value!($iter, $t)
		input_inner!($iter $($r)*)
	};
}

#[snippet = "template"]
#[allow(unused_macros)]
macro_rules! read_value {
	($iter:expr, ( $($t:tt),* )) => {
		( $(read_value!($iter, $t)),* )
	};
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
