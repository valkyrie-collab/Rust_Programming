mod tree;
use tree::binary::Binary;
use tree::node::Node;

fn main() {
    let str_arr: Vec<String> = vec![String::from("Rajarshi"), String::from("Shovona"), String::from("Arijit")];
    let mut binary_tree: Binary<&String> = Binary::new();

    for i in &str_arr {
        binary_tree.append(Box::new(Node::new(i)));
    }

    binary_tree.show_tree();

    
}

