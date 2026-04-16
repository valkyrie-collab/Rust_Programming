struct ImportantExpert<'a> {
    part: &'a str
}

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {

    if x.len() > y.len() {
        x
    } else {
        "cannot be define"
    }

}

// fn longest_two(x: &str, y: &str) -> &str { without lifetime we cannot send reference return type as a return type like &str &i32 etc

//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
    
// }

fn main() {
    let string1: String = String::from("long string in long");
    let string2: String = String::from("xyz");
    // let result: &str = longest_two(&string1, &string2);
    
    {
        let string2: String = String::from("xyz");
        // result = longest(string1.as_str(), string2.as_str());
    }

    // println!("The longest string {}", result);

    let novel: String = String::from("Call me Ishmael. Some years ago...");
    let first: &str = novel.split('.').next().expect("cannot find .");
    let i: ImportantExpert = ImportantExpert { part: &novel };
}
