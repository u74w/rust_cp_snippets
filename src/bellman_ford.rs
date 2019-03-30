use std::usize;
use std::isize;

#[snippet = "bellman_ford"]
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: isize,
    position: usize,
}

#[snippet = "bellman_ford"]
#[derive(Clone)]
struct Edge {
    node: usize,
    cost: isize,
}

#[snippet = "bellman_ford"]
struct BellmanFord<'a> {
    adj_list     : &'a Vec<Vec<Edge>>,
    dist         : Vec<isize>,
    prev         : Vec<usize>,
    start        : usize,
    negative_loop: bool
}

#[snippet = "bellman_ford"]
impl<'a> BellmanFord<'a> {
    #[allow(dead_code)]
    fn new(adj_list: &'a Vec<Vec<Edge>>, start: usize) -> Self {
        BellmanFord {
            adj_list     : adj_list,
            dist         : (0..adj_list.len()).map(|_| isize::MAX).collect(),
            prev         : (0..adj_list.len()).map(|_| usize::MAX).collect(),
            start        : start,
            negative_loop: false
        }
    }

    #[allow(dead_code)]
    fn shortest_dist(&mut self, goal: usize) -> Option<isize> {
        if !self.negative_loop && self.dist[self.start] == 0 { return Some(self.dist[goal]); }
        self.dist[self.start] = 0;
        for cnt in 0..self.adj_list.len() {
            let mut update = false;
            for i in 0..self.adj_list.len() {
                let State { cost, position } = State { cost: self.dist[i], position: i };
                if cost == isize::MAX { continue; }
                for edge in &self.adj_list[i] {
                    let next = State { cost: cost + edge.cost, position: edge.node };
                    if next.cost < self.dist[next.position] {
                        self.dist[next.position] = next.cost;
                        self.prev[next.position] = position;
                        update = true;
                        if cnt == self.adj_list.len() - 1 {
                            self.negative_loop = true;
                            return None;
                        }
                    }
                }
            }
            if !update { break; }
        }
        match self.dist[goal] {
            isize::MAX => None,
            _          => Some(self.dist[goal])
        }
    }

    #[allow(dead_code)]
    fn shortest_path(&self, goal: usize) -> Option<Vec<usize>> {
        if self.negative_loop || self.dist[goal] == isize::MAX { return None; }
        let mut path = Vec::new();
        let mut p = goal;
        while p != usize::MAX {
            path.push(p);
            p = self.prev[p];
        }
        path.reverse();
        Some(path)
    }
}

#[test]
fn test_bellman_ford() {
    //                  7
    //          +-----------------+
    //          |                 |
    //          v   1        2    |  2
    //          0 -----> 1 -----> 3 ---> 4
    //          |        ^        ^      ^
    //          |        | 1      |      |
    //          |        |        | 3    | 1
    //          +------> 2 -------+      |
    //           -11     |               |
    //                   +---------------+
    //
    let graph = vec![
            // Node 0
            vec![Edge { node: 2, cost: -11 },
                Edge { node: 1, cost: 1 }],
            // Node 1
            vec![Edge { node: 3, cost: 2 }],
            // Node 2
            vec![Edge { node: 1, cost: 1 },
                Edge { node: 3, cost: 3 },
                Edge { node: 4, cost: 1 }],
            // Node 3
            vec![Edge { node: 0, cost: 7 },
                Edge { node: 4, cost: 2 }],
            // Node 4
            vec![]
    ];

    let mut bellman_ford = BellmanFord::new(&graph, 0);
    assert_eq!(bellman_ford.shortest_dist(1), None);
    assert_eq!(bellman_ford.shortest_path(1), None);
    assert_eq!(bellman_ford.shortest_dist(3), None);
    assert_eq!(bellman_ford.shortest_path(3), None);
    let mut bellman_ford = BellmanFord::new(&graph, 4);
    assert_eq!(bellman_ford.shortest_dist(0), None);
    assert_eq!(bellman_ford.shortest_path(0), None);

    //                  7
    //          +-----------------+
    //          |                 |
    //          v   1        2    |  2
    //          0 -----> 1 -----> 3 ---> 4
    //          |        ^        ^      ^
    //          |        | 1      |      |
    //          |        |        | 3    | 1
    //          +------> 2 -------+      |
    //           -2      |               |
    //                   +---------------+
    //
    let graph = vec![
            // Node 0
            vec![Edge { node: 2, cost: -2 },
                Edge { node: 1, cost: 1 }],
            // Node 1
            vec![Edge { node: 3, cost: 2 }],
            // Node 2
            vec![Edge { node: 1, cost: 1 },
                Edge { node: 3, cost: 3 },
                Edge { node: 4, cost: 1 }],
            // Node 3
            vec![Edge { node: 0, cost: 7 },
                Edge { node: 4, cost: 2 }],
            // Node 4
            vec![]
    ];
    let mut bellman_ford = BellmanFord::new(&graph, 0);
    assert_eq!(bellman_ford.shortest_dist(1), Some(-1));
    assert_eq!(bellman_ford.shortest_path(1), Some(vec![0, 2, 1]));
    assert_eq!(bellman_ford.shortest_dist(3), Some(1));
    assert_eq!(bellman_ford.shortest_path(3), Some(vec![0, 2, 3]));
    let mut bellman_ford = BellmanFord::new(&graph, 4);
    assert_eq!(bellman_ford.shortest_dist(0), None);
    assert_eq!(bellman_ford.shortest_path(0), None);
}
