use std::process;
use std::time::SystemTime;
use std::io::{ Read, Write };
use std::fs::{OpenOptions, File, write};
use std::env::{ self, Args };

enum FileOpenType {
    Create,
    Read,
    Update,
    Delete
}

fn type_of_file(log_file: &mut File, args: Vec<String>) -> (File, FileOpenType) {
    let mut ops: OpenOptions = OpenOptions::new();
    let file: File;
    let fot: FileOpenType;

    if args[2] == "-c" {
        fot = FileOpenType::Create;
        file = ops.create(true).write(true).open(&args[1]).unwrap_or_else(|e| {
            writeln!(log_file, "At {:?} this Err: {} Occured", SystemTime::now(), e).unwrap();
            process::exit(1);
        });
    } else if args[2] == "-r" {
        fot = FileOpenType::Read;
        file = ops.read(true).open(&args[1]).unwrap_or_else(|e| {
            writeln!(log_file, "At {:?} this Err: {} Occured", SystemTime::now(), e).unwrap();
            process::exit(1);
        });
    } else if args[2] == "-a" {
        fot = FileOpenType::Update;
        file = ops.create(true).read(true).append(true).open(&args[1]).unwrap_or_else(|e| {
            writeln!(log_file, "At {:?} this Err: {} Occured", SystemTime::now(), e).unwrap();
            process::exit(1);
        });
    } else if args[2] == "-d" {
        fot = FileOpenType::Delete;
        file = ops.write(true).truncate(true).open(&args[1]).unwrap_or_else(|e| {
            println!("There is no such file to truncate/ remove");
            writeln!(log_file, "At {:?} this Err: {} Occured", SystemTime::now(), e).unwrap();
            process::exit(1);
        });
    } else {
        writeln!(log_file, "At {:?} this Err: There is no such operation {} Occured", SystemTime::now(), args[2]).unwrap();
        process::exit(1);
    }

    (file, fot)
}

fn init_log_file() -> File {
    let log_file: File = OpenOptions::new().append(true).open("log.txt").unwrap_or_else(|err| {
        println!("Err: {}", err);
        process::exit(1);
    });

    log_file
}

fn init_env_args(log: &mut File) -> Option<Vec<String>> {
    let o_args: Args = env::args();
    let args: Vec<String> = o_args.collect();

    if args.len() != 3 {
        writeln!(log, "At {:?} this Err: There is very few / large number of args Occured", SystemTime::now()).unwrap();
        return None;
    }

    Some(args)
}

fn do_file_operation(file: &mut File, log_file: &mut File, file_open: FileOpenType) {

    match file_open {
        FileOpenType::Create => { println!("Created new file"); }
        FileOpenType::Read => {
            let mut content: String = String::new();
            file.read_to_string(&mut content).expect("Err: Cannot read from file");
            println!("{}", content);
        }
        FileOpenType::Update => {
            print!("Enter the content -> ");
            std::io::stdout().flush().expect("Cannot clear newline");
            let mut content: String = String::new();
            std::io::stdin().read_line(&mut content).unwrap_or_else(|err| {
                println!("Err: {}", err);
                writeln!(log_file, "Err: {}", err).expect("cannot write in log file try again");
                process::exit(1);
            });
            write!(file, "{}", content).expect("Err: Cannot write into the file");
        }
        FileOpenType::Delete => { println!("Deleted all content in the file"); }
    }
    
}

fn main() {
    let mut log_file: File = init_log_file();
    let args: Vec<String> = init_env_args(&mut log_file).unwrap_or_else(|| {
        process::exit(1);
    });
    let (mut file, file_open): (File, FileOpenType) = type_of_file(&mut log_file, args);

    do_file_operation(&mut file, &mut log_file, file_open);
}
