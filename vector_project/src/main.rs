enum SpreadSheetCell {
    Int(i32),
    Float(f32),
    Text(String)
}

fn vec_value(arr: Vec<i32>) {

    for i in &arr {
        println!("{}", i);
    }

    println!("value of arr[0]: {}", arr[0]);
    println!("value of arr[1]: {}", arr[1]);
}

fn vec_new_value(arr: &mut Vec<i32>) {

    for i in &*arr {
        println!("{}", i);
    }

    arr.push(5);

    for i in arr {
        println!("{}", i);
    }

}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v2: Vec<i32> = vec![1, 2, 3];
    let v3: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut v4: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    let v5: Vec<SpreadSheetCell> = vec![SpreadSheetCell::Int(32), 
                                        SpreadSheetCell::Float(45.89), 
                                        SpreadSheetCell::Text(String::from("value"))];
    let len: usize = 5;
    let v6: &mut Vec<i32> = &mut vec![0; len];
    let num: i32 = -1;

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    v6.push(9);

    for i in &*v6 {
        println!("{}", i);
    }

    let v7: &mut Vec<i32> = v6;

    vec_value(v2);

    let third: i32 = v[0];
    println!("The value of first third: {}", third);

    let third: Option<&i32> = v.get(4);
    println!("The value of second third: {}", match third {
        Some(n) => n,
        None => &num
    });

    let four: i32 = v3[3];
    println!("The value is first four: {}", four);

    // let four: i32 = v3[100];
    // println!("The value of second four: {}", four);

    let four: Option<&i32> = v3.get(100);
    println!("The value of second four: {}", match four {
        None => &num,
        Some(n) => n
    });

    let five: i32 = v4[0];
    v4.push(8);
    println!("The value of first five:{}", five);

    // let five: &i32 = &v4[1];
    // v4.push(9);
    // println!("The value of second five: {}", five);

    vec_new_value(&mut v4);
}
