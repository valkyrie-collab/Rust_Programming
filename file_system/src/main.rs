use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::io::{Seek, SeekFrom};
use std::process;

fn main() {
    let mut file: File = File::open("hello.txt").unwrap_or_else(|err| {
        eprintln!("There is no such file to read {}", err);
        process::exit(1);
    });

    let mut file_two: File = OpenOptions::new().append(true).read(true).open("output.txt").unwrap_or_else(
        |err| {
            println!("Unexpected Err: {}", err);
            process::exit(1);
        }
    );

    let mut srt_wrd: String = String::new();

    file.read_to_string(&mut srt_wrd).expect("cannot read from file");

    println!("value: {}", srt_wrd);
    writeln!(&mut file_two, "{} ",srt_wrd).unwrap_or_else(|err| {
        println!("Err writing in file: The err is: {}", err);
        process::exit(1);
    });
    write!(&mut file_two, "{} ",srt_wrd).unwrap_or_else(|err| {
        println!("Err writing in file: The err is: {}", err);
        process::exit(1);
    });

    srt_wrd = String::new();
    file_two.seek(SeekFrom::Start(0)).unwrap_or_else(|err| {
        println!("cannot move cursor: {}", err);
        process::exit(1);
    });

    file_two.read_to_string(&mut srt_wrd).unwrap_or_else(|err| {
        println!("The value in the file output.txt: {}", err);
        process::exit(1);
    }); 

    println!("{}", srt_wrd);

    let mut file_three: File = OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open("new_file.txt").unwrap_or_else(|err| {
            println!("The err is: {}", err);
            process::exit(1);
        });

    writeln!(&mut file_three, "{} ", srt_wrd).unwrap_or_else(|err| {
        println!("The err is that we cannot write in file three: {}", err);
        process::exit(1);
    });

}
