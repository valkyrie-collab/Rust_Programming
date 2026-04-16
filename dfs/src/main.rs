struct Node {
    vertex: usize,
    edges: Vec<usize>
}

struct Graph {
    graph: Vec<Node>
}

impl Node {
    fn new(ver: usize, size: usize) -> Self {
        let eds: Vec<usize> = vec![0; size];
        Node { vertex: ver, edges: eds }
    }
}

impl Graph {
    fn new(size: usize) -> Self {
        let mut grp: Vec<Node> = Vec::new();
        let mut i: usize = 0;

        while i < size {
            grp.push(Node::new(i, size));
            i += 1;
        }

        Graph { graph: grp }
    }

    fn insert(&mut self, ver: usize, edge: usize) {
        let mut i: usize = 0;
        let grp: &mut Vec<Node> = &mut self.graph;
        let len: usize = grp.len();
        let edges: &mut Vec<usize> = &mut grp[ver].edges;

        while i < len {

            if edges[i] == edge {
                break;
            }

            if edges[i] == 0 {
                edges[i] = edge;
                break;
            }

            i += 1;
        }

    }

    fn show(&self) {
        let grp: &Vec<Node> = &self.graph;
        let mut eds: &Vec<usize>;

        for i in grp {
            print!("{} -> [ ", i.vertex);
            eds = &i.edges;

            for k in eds {
                print!(" {} ", *k);
            }

            println!(" ]")
        }

        println!("\n");
    }
}

fn rec_dfs(graph: &Graph, srt_index: usize, visited: &mut Vec<bool>, len: usize) {
    let eds: &Vec<usize> = &graph.graph[srt_index].edges;
    let mut index: usize = 0;
    visited[srt_index] = true;
    print!("{} -> ", srt_index);

    while index < len {

        if eds[index] != 0 && !visited[eds[index]] {
            rec_dfs(graph, eds[index], visited, len);
        }

        index += 1;
    }    

}

fn do_dfs(graph: &Graph, srt_index: usize) {
    let len: usize = graph.graph.len();
    println!("size of len: {}", len);
    let mut visited: Vec<bool> = vec![false; len];

    rec_dfs(graph, srt_index, &mut visited, len);
    println!();
}

fn main() {
    let mut graph: Graph = Graph::new(5);
    graph.show();

    graph.insert(1, 2);
    graph.insert(1, 4);
    graph.insert(0, 3);
    graph.insert(3, 1);

    graph.show();

    do_dfs(&graph, 0);
}
