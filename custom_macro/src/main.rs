macro_rules! my_vec {
    ($($value: expr), *) => {
        {
            let mut new_vec: Vec<usize> = Vec::new();

            $(new_vec.push($value);)*
            new_vec
        }
    };
}

fn main() {
    println!("Hello, world!");

    let main_vec: Vec<usize> = my_vec![1, 2, 3, 5];

    println!("{:?}", main_vec);

}
