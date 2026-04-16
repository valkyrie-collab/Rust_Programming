fn main() {
    let mut s: String = String::new();
    let data: &str = "initial contents";
    let s: String = data.to_string();
    let s2: String = "initial contents".to_string();
    let s3: String = String::from("initial contents");
    let s4: String = String::from("নমস্কার");
    let mut s6: String = String::from("foo");
    s6.push_str("bar");
    s6.push('c');

    let s7: &str = "bar";

    s6.push_str(s7);
    s6.push_str(&s4);

    let s8: String = String::from("Hello, ");
    let s9: String = String::from("world!");
    let s10: String = s8 + &s9;
    println!("{}", s10);

    let s11: String = String::from("tic");
    let s12: String = String::from("tac");
    let s13: String = String::from("tao");
    let s14: String = s11 + "_" + &s12 + "_" + &s13;
    println!("{}", s14);

    let s16: String = String::from("tic");
    let s15: String = format!("{}-{}-{}", s16, s12, s13);
    println!("{}", s15);

    for i in s15.chars() {
        println!("{}", i);
    }

    for i in s15.bytes() {
        println!("{}", i);
    }
}

