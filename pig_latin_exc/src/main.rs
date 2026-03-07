fn main() {
    let str_wrd: String = String::from("apple");
    let arr: [u8; 26] = [
        1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0
    ];

    if let Some(alpha) = str_wrd.chars().next() {

        if (arr[(alpha as usize) - 97]) == 1 {
            println!("{}", str_wrd + "-hay")
        } else {
            let new_str_wrd: String = format!("{}-{}ay", &str_wrd[1..str_wrd.len()], alpha);
            println!("{}", new_str_wrd);
        }
    }

}
