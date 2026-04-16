use std::{collections::VecDeque, i32};

struct Graph {
    grp: Vec<Vec<i32>>
}

impl Graph {
    fn new(size: usize) -> Self {
        let mut i: usize = 0;
        let mut grp: Vec<Vec<i32>> = vec![Vec::new(); size];

        while i < size {
            grp[i] = vec![-1; size];
            i += 1;
        }

        Graph { grp }
    }

    fn insert(&mut self, ver: usize, ed: usize, cost: i32) {
        let grp: &mut Vec<i32> = &mut self.grp[ver];

        if grp[ed] != -1 {
            println!("The edge already has been visited");
            return;
        }

        grp[ed] = cost;
    }

    fn show(&self) {
        let mut g: usize = 0;

        for i in &self.grp {
            print!("{} -> [", g);

            for k in i {
                print!(" {} ", k);
            }

            println!("]");
            g += 1;
        }

    }
}

fn do_dijkstra_search(graph: &Graph, srt_pnt: usize, size: usize) {
    let mut each_node_cost: Vec<i32> = vec![i32::MAX; size];
    let mut que: VecDeque<usize> = VecDeque::new();
    let mut visited: Vec<bool> = vec![false; size];
    let mut path: Vec<i32> = vec![-1; size];
    let mut i: usize = 0;
    let mut prev_sm_node: usize = 0;

    que.push_back(srt_pnt);
    visited[srt_pnt] = true;
    each_node_cost[srt_pnt] = 0;

    while let Some(node) = que.pop_front() {
        let eds: &Vec<i32> = &graph.grp[node];
        let mut min: i32 = i32::MAX;

        for (n, c) in eds.iter().enumerate() {

            if *c != -1 {
                let tlt_cost: i32 = each_node_cost[node] + *c;
                // println!("tlt_cost: {}", tlt_cost);

                if tlt_cost < each_node_cost[n] {
                    each_node_cost[n] = tlt_cost;
                }

                if min > tlt_cost {

                    if prev_sm_node == 0 {
                        path[i] = n as i32;
                        prev_sm_node = n;
                    } else if prev_sm_node == node {
                        path[i] = n as i32;
                        prev_sm_node = n;
                    }

                    min = tlt_cost;
                }

                if !visited[n] {
                    que.push_back(n);
                    visited[n] = true;
                }

            }

        }

        i += 1;

        // println!("queue: {:?}", que);
        // println!("for loop completer");
    }

    println!("{:?}", each_node_cost);
    println!("The path is {:?}", path);
}

fn main() {
    let mut graph: Graph = Graph::new(5);
    graph.show();

    graph.insert(0, 0, 0);
    graph.insert(0, 1, 10);
    graph.insert(0, 2, 7);
    graph.insert(1, 3, 5);
    graph.insert(1, 2, 5);
    graph.insert(1, 4, 12);
    // graph.insert(2, 1, 5);
    // graph.insert(2, 0, 7);
    // graph.insert(2, 3, 1);
    graph.insert(2, 4, 9);
    graph.insert(3, 4, 10);
    // graph.insert(3, 1, 5);
    // graph.insert(4, 3, 10);
    // graph.insert(4, 2, 9);
    // graph.insert(4, 1, 12);

    println!("\n");
    graph.show();

    do_dijkstra_search(&graph, 0, 5);
}