use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    let teams: Vec<String> = vec![String::from("blue"), String::from("red")];
    let init_scores: Vec<u32> = vec![10, 20];
    let scores_two: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();

    let field_name: String = String::from("favorite color");
    let field_value: String = String::from("blue");
    let mut map: HashMap<String, String> = HashMap::new();

    map.insert(field_name, field_value);

    // let field_name_two: String = field_name + &field_value;

    let field_name: String = String::from("favorite color");
    let field_value: Option<&String> = map.get(&field_name);

    if let Some(f_v) = field_value {
        println!("{}", f_v);
    } else {
        println!("NONE");
    }

    for (k, v) in &map {
        println!("Key: {}, Value: {}", k, v);
    }

    map.insert(field_name, "green".to_string());

    let field_name: String = String::from("favorite color");
    let field_value: Option<&String> = map.get(&field_name);

    if let Some(f) = field_value {
        println!("{}", f);
    } else {
        println!("NONE");
    }

    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.entry(String::from("yellow")).or_insert(50);
    scores.entry(String::from("blue")).or_insert(50);

    for (k, v) in scores {
        println!("{} {}", k, v);
    }

    let text: &str = "hello world hello world";
    let mut map: HashMap<String, u32> = HashMap::new();
    let num: u32 = 0;

    for word in text.split_whitespace() {
        let count: &mut u32 = map.entry(word.to_string()).or_insert(num);
        *count += 1;
    }

    for (k, v) in map {
        println!("{} {}", k, v);
    }

    println!("num {}", num);
}

