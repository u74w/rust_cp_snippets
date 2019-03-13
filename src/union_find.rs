#[snippet = "UnionFind"]
#[allow(dead_code)]
///Referenced from https://github.com/hatoo/competitive-rust-snippets
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>
}

#[snippet = "UnionFind"]
impl UnionFind {
    #[allow(dead_code)]
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n]
        }
    }
    #[allow(dead_code)]
    fn root(&mut self, x: usize) -> usize {
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
    fn unite(&mut self, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);
        if x == y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.parent[x] = y;
            self.size[y] += self.size[x];
        } else {
            self.parent[y] = x;
            self.size[x] += self.size[y];
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }
    #[allow(dead_code)]
    fn size(&mut self, x: usize) -> usize {
        let r = self.root(x);
        self.size[r]
    }
    #[allow(dead_code)]
    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
}

#[test]
fn test_union_find() {
    let mut uf = UnionFind::new(5);
    let info = vec![(0, 1), (1, 2), (3, 4)];
    assert_eq!(uf.size(0), 1);
    for (i, j) in info {
        uf.unite(i, j);
    }
    assert!(uf.is_same(0, 1));
    assert_eq!(uf.root(2), 0);
    assert_eq!(uf.root(3), 3);
    assert_eq!(uf.size(0), 3);
    uf.unite(0, 4);
    assert!(uf.is_same(0, 4));
    assert_eq!(uf.size(0), 5);
}
