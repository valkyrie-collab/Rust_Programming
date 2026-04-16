mod list;
use list::double::Double;
use list::single::Single;

fn main() {
    let mut single_list: Single<String> = Single::new();
    let mut double_list: Double<u32> = Double::new();
    let str_arr: Vec<String> = vec![String::from("Rajarshi"), String::from("Shovona"), String::from("Arijit")];
    let num_arr: Vec<u32> = vec![1, 2, 3, 4, 5];

    for i in str_arr {
        single_list.append(i);
    }

    for i in num_arr {
        double_list.append(i);
    }
    
    single_list.show_list();
    double_list.show_list();
    println!();

    let str_arr: Vec<String> = single_list.retrive();
    println!("{:?}", str_arr);
}
