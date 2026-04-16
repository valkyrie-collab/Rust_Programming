use std::collections::HashSet;

fn is_anagram_string(word_one: String, word_two: String) -> bool {
    let mut hash_set: HashSet<char> = HashSet::with_capacity(word_one.len() + 1);

    for i in word_one.chars() {
        hash_set.insert(i);
    }

    for i in word_two.chars() {

        if !hash_set.contains(&i) {
            return false;
        }

    }

    true
}

fn main() {
    let mut word_one: String = String::new();
    std::io::stdin().read_line(&mut word_one).expect("cannot read from the terminal for word one");
    word_one = word_one.trim().parse().unwrap();

    let mut word_two: String = String::new();
    std::io::stdin().read_line(&mut word_two).expect("cannot read from the terminal for word two");
    word_two = word_two.trim().parse().unwrap();

    if word_one.len() != word_two.len() {
        println!("The word is anagram the statement is : false");
    }

    let is_ana: bool = is_anagram_string(word_one, word_two);

    println!("The word is anagram the statement is : {}", is_ana);
}
