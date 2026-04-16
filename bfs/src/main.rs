use std::collections::VecDeque;

struct Node {
    vertex: u32,
    edges: Vec<u32>
}

struct Graph {
    graph: Vec<Node>
}

impl Node {
    fn new(size: usize, ver: u32) -> Self {
        Node { vertex: ver, edges: vec![0; size] }
    }
}

impl Graph {
    fn new(size: usize) -> Self {
        let mut i: usize = 0;
        let mut grp: Vec<Node> = Vec::new();

        while i < size {
            grp.push(Node::new(size, i as u32));
            i += 1;
        }

        Graph { graph: grp }
    }

    fn insert(&mut self, ver: u32, ed: u32) {
        let mut i: usize = 0;
        let len: u32 = self.graph.len() as u32;

        if ver >= len || ed >= len {
            return;
        }

        let node: &mut Node = &mut self.graph[ver as usize];
        let edge: &mut Vec<u32> = &mut node.edges;

        while i < len as usize {

            if edge[i] == ed {
                break;
            }

            if edge[i] == 0 {
                edge[i] = ed;
                break;
            }

            i += 1;
        }

    }

    fn show(&self) {
        let grp: &Vec<Node> = &self.graph;
        let mut ver: &Vec<u32>;

        for node in grp {
            ver = &node.edges;
            print!("{} -> [", node.vertex);

            for edge in ver {
                print!(" {} ", edge);
            }

            println!("]");
        }

    }
}

fn do_bfs(graph: &Graph, srt_node: u32, size: usize) {
    let mut visited: Vec<bool> = vec![false; size];
    let mut que: VecDeque<u32> = VecDeque::new();
    let grp: &Vec<Node> = &graph.graph;
    let mut grp_elm: &Vec<u32>;

    visited[srt_node as usize] = true;
    que.push_front(srt_node);

    while let Some(element) = que.pop_front() { 
        print!("{} -> ", element);

        grp_elm = &grp[element as usize].edges;

        for i in grp_elm {

            if !visited[*i as usize] {
                que.push_back(*i);
                visited[*i as usize] = true;
            }

        }

    }

    println!("FIN")
}

fn main() {
    let size: usize = 5;
    let mut grp: Graph = Graph::new(size);
    grp.show();
    grp.insert(0, 1);
    grp.insert(0, 2);
    grp.insert(1, 2);
    grp.insert(1, 3);
    grp.insert(2, 3);
    grp.insert(3, 4);
    println!("\n");
    grp.show();
    
    do_bfs(&grp, 0, size);
}
