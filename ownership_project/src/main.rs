fn takes_ownership(some_str: String) {
    println!("{}", some_str);
}

fn takes_int_ownership(some_int: i32) {
    println!("{}", some_int);
}

fn takes_and_gives(wrd: String) -> String{
    println!("Printing {} from function", wrd);

    wrd
}

fn cal_len(wrd: String) -> (usize, String) {
    let len: usize = wrd.len();

    (len, wrd)
}

fn cal_only_len(wrd: &String) -> usize {
    wrd.len()
}

fn modify_borrow(wrd: &mut String) {
    wrd.push_str("hello");
}

fn no_dangling_pointer() -> String {
    let wrd: String = String::from("hello");

    wrd
}

fn first_wrd(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() { //enumerate adds index

        if item == b' ' { //b'<any alpha>' give ascii value
            return i;
        }

    }

    return s.len();
}

fn second_wrd(wrd: &String) -> &str {
    let bytes: &[u8] = wrd.as_bytes();

    for (i, &items) in bytes.iter().enumerate() {

        if items == b' ' {
            return &wrd[0..i];
        }

    }

    return &wrd[0..]; // or &wrd[..]
}

fn main() {
    let x: String = String::from("Hello");
    let mut y: String = x.clone();
    let n: i32 = 6;

    println!("{}, World", x);

    y = takes_and_gives(y);

    takes_ownership(y);
    takes_int_ownership(n);

    // println!("The value of y: {}", y);
    println!("The value of n: {}", n);

    let x: String = String::from("hello");
    let tup: (usize, String) = cal_len(x);

    println!("The word is {} and its size {} ", tup.1, tup.0);

    let x: String = String::from("world");
    let len: usize = cal_only_len(&x);

    println!("The word is {} and its size is {}", x, len);

    let mut x: String = String::from("world");
    modify_borrow(&mut x);

    println!("The new String is {}", x);

    let wrd: String = no_dangling_pointer();

    println!("The no dangling pointer String: {}", wrd);

    let len: usize = first_wrd(&wrd);

    println!("The value of first word: {}", len);

    let s: String = String::from("hello_world");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    let y: usize = s.len();

    let first: &str = &s[0..y];
    println!("The value of first {}", first);

    let second: &str = &s[0..];
    println!("The value of second {}", second);

    let third: &str = &s[3..y];
    println!("The of the third: {}", third);

    println!("The value of hello: {}", hello);
    println!("The value of world: {}", world);

    let s: String = String::from("he llo world");
    let sd_wrd: &str = second_wrd(&s);
    println!("The value of second word {}", sd_wrd);
}
