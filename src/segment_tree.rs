use cargo_snippet::snippet;

#[snippet("seg")]
trait Monoid {
    type T: Clone;
    fn id() -> Self::T;
    fn op(a: &Self::T, b: &Self::T) -> Self::T;
}

#[snippet("seg")]
struct SegmentTree<M: Monoid> {
    n_leaf: usize,
    data  : Vec::<M::T>,
}

#[snippet("seg")]
impl<M: Monoid> SegmentTree<M> {
    #[allow(dead_code)]
    fn new(_n: usize) -> SegmentTree<M> {
        let mut n = 1;
        while n < _n {
            n *= 2;
        }

        SegmentTree {
            n_leaf: n,
            data  : vec![M::id(); 2 * n - 1],
        }
    }

    #[allow(dead_code)]
    fn update(&mut self, i: usize, val: M::T) {
        let mut i = i + self.n_leaf - 1;
        self.data[i] = val;

        while i > 0 {
            i = (i - 1) / 2;
            self.data[i] = M::op(&self.data[2 * i + 1], &self.data[2 * i + 2]);
        }
    }

    #[allow(dead_code)]
    fn _query(&self, left: usize, right: usize, i: usize, l: usize, r: usize) -> M::T {
        if right <= l || r <= left {
            M::id()
        } else if left <= l && r <= right {
            self.data[i].clone()
        } else {
            let child_left  = self._query(left, right, i * 2 + 1, l, (l + r) / 2);
            let child_right = self._query(left, right, i * 2 + 2, (l + r) / 2, r);
            M::op(&child_left, &child_right)
        }
    }

    #[allow(dead_code)]
    fn query(&self, left: usize, right: usize) -> M::T {
        self._query(left, right, 0, 0, self.n_leaf)
    }
}

#[snippet("seg")]
impl<M: Monoid> std::ops::Index<usize> for SegmentTree<M> {
    type Output = M::T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index + self.n_leaf - 1]
    }
}

#[snippet("segmini64")]
enum MinI64 {}

#[snippet("segmini64")]
impl Monoid for MinI64 {
    type T = i64;
    fn id() -> Self::T {
        Self::T::max_value()
    }
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        std::cmp::min(*a, *b)
    }
}

#[snippet("segminf64")]
enum MinF64 {}

#[snippet("segminf64")]
impl Monoid for MinF64 {
    type T = f64;
    fn id() -> Self::T {
        std::f64::INFINITY
    }
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        a.min(*b)
    }
}

#[snippet("segsumi64")]
enum SumI64 {}

#[snippet("segsumi64")]
impl Monoid for SumI64 {
    type T = i64;
    fn id() -> Self::T {
        0
    }
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        *a + *b
    }
}

#[snippet("segsumf64")]
enum SumF64 {}

#[snippet("segsumf64")]
impl Monoid for SumF64 {
    type T = f64;
    fn id() -> Self::T {
        0.0
    }
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        *a + *b
    }
}

#[test]
fn test_segment_tree() {
    let n = 1024;
    let mut sgt: SegmentTree<MinI64> = SegmentTree::new(n);
    for i in 0..n {
        sgt.update(i, i as i64);
        assert_eq!(sgt[i], i as i64);
    }

}