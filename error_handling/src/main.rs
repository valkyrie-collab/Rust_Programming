use std::fs::File;
use std::fs::read_to_string;
use std::io::Error;
use std::io::Read;
use std::io::ErrorKind;
use std::cmp::Ordering;

fn read_file_content() -> Result<String, Error> {
    let f: Result<File, Error> = File::open("hello.txt");

    let mut f: File = match f {
        Ok(file) => file,
        Err(e) => {
            panic!("cannot open file {:?}", e);
        }
    };

    let mut str_wrd: String = String::new();

    match f.read_to_string(&mut str_wrd) {
        Ok(_) => Ok(str_wrd),
        Err(e) => Err(e)
    }
}

fn read_file_content_two() -> Result<String, Error> {
    let mut str_wrd: String = String::new();

    File::open("hello.txt")?.read_to_string(&mut str_wrd)?;
    Ok(str_wrd)
}

fn main() {
    // panic!("crash and burn");

    // let v: Vec<i32> = vec![1, 2, 3];
    // v[99];

    // let f: Result<File, Error> = File::open("hello.txt");

    // let f: File = match f {
    //     Ok(file) => file,
    //     Err(err) if err.kind() == ErrorKind::NotFound => {
    //         match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!("Tried to create file {:?}", e);
    //             }
    //         }
    //     }
    //     Err(err) => {
    //         panic!("There is a problem with opening file: {:?}", err);
    //     }
    // };

    // let f: File = File::open("hello.txt").unwrap();
    
    // let f: File = match f {
    //     Ok(f) => f,
    //     Err(e) => {
    //         panic!("There was a problem openning the file");
    //     }
    // };

    println!("{}", match read_file_content_two() {
        Ok(s) => s,
        Err(_) => String::from("none")
    })
}
