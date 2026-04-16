mod list;
use list::List;

fn main() {
    let mut list: List = List::new();
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    for i in arr {
        list.insert(i);
    }

    list.show_list();
}
