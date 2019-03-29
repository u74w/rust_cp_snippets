use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;

#[snippet = "dijkstra"]
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

#[snippet = "dijkstra"]
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        match other.cost.cmp(&self.cost) {
            Ordering::Equal => self.position.cmp(&other.position),
            _               => other.cost.cmp(&self.cost)
        }
    }
}

#[snippet = "dijkstra"]
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[snippet = "dijkstra"]
#[derive(Clone)]
struct Edge {
    node: usize,
    cost: usize,
}

#[snippet = "dijkstra"]
struct Dijkstra<'a> {
    adj_list: &'a Vec<Vec<Edge>>,
    dist: Vec<usize>,
    prev: Vec<usize>,
    start: usize,
    goal: usize
}

#[snippet = "dijkstra"]
impl<'a> Dijkstra<'a> {
    #[allow(dead_code)]
    fn new(adj_list: &'a Vec<Vec<Edge>>, start: usize, goal: usize) -> Self {
        Dijkstra {
            adj_list: adj_list,
            dist: (0..adj_list.len()).map(|_| usize::MAX).collect(),
            prev: (0..adj_list.len()).map(|_| usize::MAX).collect(),
            start: start,
            goal: goal
        }
    }

    #[allow(dead_code)]
    fn shortest_dist(&mut self) -> Option<usize> {
        let mut heap = BinaryHeap::new();
        self.dist[self.start] = 0;
        heap.push(State { cost: 0, position: self.start });
        while let Some(State { cost, position }) = heap.pop() {
            if position == self.goal { return Some(cost); }
            if cost > self.dist[position] { continue; }

            for edge in &self.adj_list[position] {
                let next = State { cost: cost + edge.cost, position: edge.node };
                if next.cost < self.dist[next.position] {
                    heap.push(next);
                    self.dist[next.position] = next.cost;
                    self.prev[next.position] = position;
                }
            }
        }
        None
    }

    #[allow(dead_code)]
    fn shortest_path(&self) -> Option<Vec<usize>> {
        if self.dist[self.goal] == usize::MAX { return None; }
        let mut path = Vec::new();
        let mut p = self.goal;
        while p != usize::MAX {
            path.push(p);
            p = self.prev[p];
        }
        path.reverse();
        Some(path)
    }
}

#[test]
fn test_dijkstra() {
    let graph = vec![
            // Node 0
            vec![Edge { node: 2, cost: 10 },
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
            vec![]];

    let mut dijkstra = Dijkstra::new(&graph, 0, 1);
    assert_eq!(dijkstra.shortest_dist(), Some(1));
    assert_eq!(dijkstra.shortest_path(), Some(vec![0, 1]));
    let mut dijkstra = Dijkstra::new(&graph, 0, 3);
    assert_eq!(dijkstra.shortest_dist(), Some(3));
    assert_eq!(dijkstra.shortest_path(), Some(vec![0, 1, 3]));
    let mut dijkstra = Dijkstra::new(&graph, 4, 0);
    assert_eq!(dijkstra.shortest_dist(), None);
    assert_eq!(dijkstra.shortest_path(), None);
}