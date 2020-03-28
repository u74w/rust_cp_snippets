use cargo_snippet::snippet;

#[snippet("template")]
#[allow(unused_imports)]
use std::io::{stdin, stdout, stderr, BufWriter, Write, StdoutLock};
#[snippet("template")]
#[allow(unused_imports)]
use std::cmp::{max, min, Ordering};
#[snippet("template")]
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[snippet("template")]
#[allow(unused_imports)]
use std::iter::FromIterator;
#[snippet("template")]
#[allow(unused_imports)]
use std::{usize, u64, i64};

#[snippet("template")]
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

#[snippet("template")]
#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };

    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[snippet("template")]
#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, [ $t:tt ]) => {
        {
            let len = read_value!($next, usize);
            (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
        }
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

#[snippet("template")]
#[allow(dead_code)]
struct Writer {
    s: String,
}

#[snippet("template")]
#[allow(unused_imports)]
use std::fmt::Display;

#[snippet("template")]
#[allow(dead_code)]
/// let mut writer = Writer::new();
/// writer.writeln(hoge);
/// writer.flush()
impl Writer {
    #[allow(dead_code)]
    pub fn new() -> Writer {
        Writer {
            s: String::new(),
        }
    }
    #[allow(dead_code)]
    pub fn flush(&mut self) {
        print!("{}", self.s);
        self.s.clear();
    }
    pub fn write<T: Display>(&mut self, x: T) {
        self.s.push_str(&format!("{}", x));
    }
    pub fn writeln<T: Display>(&mut self, x: T) {
        self.s.push_str(&format!("{}", x));
        self.s.push('\n');
    }
    #[allow(dead_code)]
    pub fn write_vec<T: Display>(&mut self, xs: &Vec<T>) {
        if xs.len() == 0 {
            self.writeln("");
            return;
        }
        self.write(&format!("{}", xs[0]));
        for i in 1..xs.len() {
            self.write(&format!(" {}", xs[i]));
        }
        self.writeln("");
    }
}

#[snippet("template")]
#[allow(unused_macros)]
macro_rules! dbg {
    ($($a:expr),*) => {
        writeln!(&mut stderr(), concat!($(stringify!($a), " = {:?}, "),*), $($a),*).unwrap();
    }
}

#[snippet("template")]
#[allow(dead_code)]
const INF_U: u64 = 1_000_000_000_000_000;

#[snippet("template")]
#[allow(dead_code)]
const INF_I: i64 = 1_000_000_000_000_000;


#[snippet("template")]
#[allow(non_snake_case)]
#[allow(dead_code)]
fn main() {
    input!{}
    let mut ans: u64;
}

#[snippet("big_stack")]
#[allow(dead_code)]
const BIG_STACK_SIZE: bool = true;

#[snippet("big_stack")]
#[allow(dead_code)]
fn big_stack() {
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

#[snippet("big_stack")]
#[allow(dead_code)]
fn solve() {}

