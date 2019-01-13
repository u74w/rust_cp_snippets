#[snippet = "UnionFind"]
#[allow(dead_code)]
pub struct UnionFind {
    pub parent: Vec<usize>,
    pub rank: Vec<usize>,
}

#[snippet = "UnionFind"]
impl UnionFind {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    #[allow(dead_code)]
    pub fn root(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let p = self.parent[x];
            let pp = self.root(p);
            self.parent[x] = pp;
            pp
        }
    }
    #[allow(dead_code)]
    pub fn unite(&mut self, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);
        if x == y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.parent[x] = y;
        } else {
            self.parent[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }
    #[allow(dead_code)]
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
}

#[test]
fn test_union_find() {
    let mut uft = UnionFind::new(5);
    let info = vec![(0, 1), (1, 2), (3, 4)];
    for (i, j) in info {
        uft.parent[j] = i;
    }
    assert!(uft.same(0, 1));
    assert_eq!(uft.root(2), 0);
    assert_eq!(uft.root(3), 3);
    uft.unite(0, 4);
    assert!(uft.same(0, 4));
}
