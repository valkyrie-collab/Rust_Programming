use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum Priority {
    High,
    Medium,
    Low
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    prio: Priority
}

impl Priority {
    fn val_to_prio(&self) -> i32 {

        match self {
            Priority::High => { 1 }
            Priority::Medium => { 0 }
            Priority::Low => { -1 }
        }

    }
}

fn merge(arr: &mut Vec<Node>, low: usize, high: usize, mid: usize) {
    let len_one: usize = mid - low + 1;
    let len_two: usize = high - mid;

    let mut arr_one: VecDeque<Node> = VecDeque::with_capacity(len_one);
    let mut arr_two: VecDeque<Node> = VecDeque::with_capacity(len_two);

    for i in 0..len_one {
        arr_one.push_back(arr[low + i].clone());
    }

    for i in 0..len_two {
        arr_two.push_back(arr[mid + i + 1].clone());
    }

    let mut k: usize = low;

    while !arr_one.is_empty() && !arr_two.is_empty() {
        let arr_one_element: Node = arr_one.pop_front().unwrap();
        let arr_two_element: Node = arr_two.pop_front().unwrap();

        if arr_one_element.prio.val_to_prio() > arr_two_element.prio.val_to_prio() {
            arr[k] = arr_one_element;
            arr_two.push_front(arr_two_element);
        } else {
            arr[k] = arr_two_element;
            arr_one.push_front(arr_one_element);
        }

        k += 1;
    }

    while !arr_one.is_empty() {
        arr[k] = arr_one.pop_front().unwrap();
        k += 1;
    }

    while !arr_two.is_empty() {
        arr[k] = arr_two.pop_front().unwrap();
        k += 1;
    }
}

fn sort(arr: &mut Vec<Node>, low: usize, high: usize) {

    if low >= high {
        return;
    }

    let mid: usize = low + (high - low) / 2;
    sort(arr, low, mid);
    sort(arr, mid + 1, high);
    merge(arr, low, high, mid);
}

fn main() {
    let node_one: Node = Node { name: String::from("Valkyrie"), prio: Priority::High };
    let node_two: Node = Node { name: String::from("Rajarshi"), prio: Priority::Medium };
    let node_three: Node = Node { name: String::from("HyperHydraX"), prio: Priority::Low };
    let node_four: Node = Node { name: String::from("Achilles"), prio: Priority::Medium };

    let mut nodes: Vec<Node> = vec![node_one, node_two, node_three, node_four];
    sort(&mut nodes, 0, 3);

    println!("{:?}", nodes);
}
