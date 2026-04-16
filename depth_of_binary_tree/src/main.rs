extern crate depth_of_binary_tree;

use depth_of_binary_tree::tree::{binary::Binary, avl::Avl, node::Node};
fn main() {
    let mut binary_tree: Binary = Binary::new();
    let mut avl_tree: Avl = Avl::new();
    let arr: Vec<(String, i32)> = vec![
        (String::from("Rajarshi"), 22), (String::from("Riya"), 20), (String::from("Shovona"), 18),
        (String::from("Rishav"), 21), (String::from("Arijit"), 23), (String::from("Ankan"), 22)
    ];

    for i in &arr {
        binary_tree.add_node(Box::new(Node::new(&i.0, i.1)));
    }

    binary_tree.show_list();

    let depth: u32 = binary_tree.find_depth();

    println!("{}", depth);

    for i in &arr {
        avl_tree.new_node(Box::new(Node::new(&i.0, i.1)));
    }

    avl_tree.show_tree();
}
