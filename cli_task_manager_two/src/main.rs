use std::collections::VecDeque;
// use serde_json::json;
use serde::{ Serialize, Deserialize };
use chrono::Local;
use std::process;
use std::fs::{ File, OpenOptions };
use std::io::{ self, Read, Write, Error };
// use std::time::{ SystemTime, UNIX_EPOCH, Duration };
use std::env::{ self, Args };

#[derive(Serialize, Deserialize, Clone, Debug)]
enum Priority {
    High,
    Medium,
    Low
}

enum Done {
    Yes,
    No,
    AlreadyDone
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Task {
    id: String,
    content: String,
    priority: Priority,
    accomplished: bool
}

impl Priority {
    fn val_to_prio(&self) -> i32{

        match self {
            Priority::High => { 1 }
            Priority::Medium => { 0 }
            Priority::Low => { -1 }
        }

    }
}

impl Task {
    fn new() -> Self {
        Task {
            id: String::new(),
            content: String::new(),
            priority: Priority::Low,
            accomplished: false
        }
    }

    fn add_err_msg(err: Error, log_file: &mut File) {
        let cur_time: String = Local::now().format("%d-%m-%Y %H:%M:%S").to_string();
        writeln!(log_file, "At {} The Err: {}", cur_time, err).unwrap();
    }

    fn add_task(mut self, log_file: &mut File, file_name: &String) {
        let mut file: File = File::open(file_name).unwrap_or_else(|e| {
            Self::add_err_msg(e, log_file);
            process::exit(1);
        });
        let mut contents: String = String::new();
        file.read_to_string(&mut contents).unwrap_or_else(|err| {
            Self::add_err_msg(err, log_file);
            process::exit(1);
        });
        let mut tasks: Vec<Task> = serde_json::from_str(&mut contents).unwrap_or_else(|err| {
            Self::add_err_msg(Error::from(err), log_file);
            process::exit(1);
        });

        file = OpenOptions::new().write(true).truncate(true).open(file_name).unwrap_or_else(|err| {
            Self::add_err_msg(err, log_file);
            process::exit(1);
        });

        loop {
            print!("Enter the Task ID: ");
            io::stdout().flush().unwrap_or_else(|err|{
                println!("Err: {}", err);
                process::exit(1);
            });
            io::stdin().read_line(&mut self.id).unwrap_or_else(|err| {
                Self::add_err_msg(err, log_file);
                process::exit(1);
            });
            self.id = self.id.trim().parse().unwrap();
            let mut is_present: bool = false;

            for t in &tasks {

                if t.id == self.id {
                    is_present = true;
                    println!("Id already present try again with new Id");
                }

            }

            if !is_present {
                break;
            }
        }

        print!("Enter the contents: ");
        io::stdout().flush().unwrap_or_else(|err| {
            Self::add_err_msg(err, log_file);
            process::exit(1);
        });
        io::stdin().read_line(&mut self.content).unwrap_or_else(|err| {
            Self::add_err_msg(err, log_file);
            process::exit(1);
        });
        self.content = self.content.trim().parse().unwrap();

        loop {
            print!("Select priority (H, M or L): ");
            io::stdout().flush().unwrap_or_else(|err| {
                Self::add_err_msg(err, log_file);
                process::exit(1);
            });
            let mut priority: [u8; 1] = [0; 1];
            io::stdin().read_exact(&mut priority).unwrap_or_else(|err| {
                Self::add_err_msg(err, log_file);
                process::exit(1);
            });
            let mut _buffer: String = String::new();
            io::stdin().read_line(&mut _buffer ).unwrap_or_else(|err| {
                Self::add_err_msg(err, log_file);
                process::exit(1);
            });
            let mut is_visited: bool = false;

            self.priority = if priority[0] == 'h' as u8 || priority[0] == 'H' as u8 {
                is_visited = true;
                Priority::High
            } else if priority[0] == 'm' as u8 || priority[0] == 'M' as u8 {
                is_visited = true;
                Priority::Medium
            } else if priority[0] == 'l' as u8 || priority[0] == 'L' as u8{
                is_visited = true;
                Priority::Low
            } else {
                Priority::Low
            };

            if is_visited {
                break;
            } else {
                println!("Select h/H or m/M or l/L only and try again");
            };

        }

        loop {
            print!("Select priority (Y or N): ");
            io::stdout().flush().unwrap_or_else(|err| {
                Self::add_err_msg(err, log_file);
                process::exit(1);
            });
            let mut acc: [u8; 1] = [0; 1];
            io::stdin().read_exact(&mut acc).unwrap_or_else(|err| {
                Self::add_err_msg(err, log_file);
                process::exit(1);
            });
            let mut _buffer: String = String::new();
            io::stdin().read_line(&mut _buffer ).unwrap_or_else(|err| {
                Self::add_err_msg(err, log_file);
                process::exit(1);
            });
            let mut is_visited: bool = false;

            self.accomplished = if acc[0] == 'y' as u8 || acc[0] == 'Y' as u8 {
                is_visited = true;
                true
            } else if acc[0] == 'n' as u8 || acc[0] == 'N' as u8 {
                is_visited = true;
                false
            } else {
                false
            };

            if is_visited {
                break;
            } else {
                println!("Select Y/n only and try again");
            }

        }

        tasks.push(self);
        serde_json::to_writer_pretty(file, &tasks).unwrap_or_else(|err| {
            Self::add_err_msg(Error::from(err), log_file);
            process::exit(1);
        });
    }

    fn prio_is(prio: &Priority) -> String {

        match prio {
            Priority::High => { String::from("High") }
            Priority::Medium => { String::from("Medium") }
            Priority::Low => { String::from("Low") }
        }

    }

    fn retrieve_task(task_id: &String, file_name: &String, log_file: &mut File) -> Option<Task> {
        let mut task: Option<Task> = None;
        let mut file: File = File::open(file_name).unwrap_or_else(|err| {
            Self::add_err_msg(err, log_file);
            process::exit(1);
        });

        let mut contents: String = String::new();
        file.read_to_string(&mut contents).unwrap_or_else(|err| {
            Self::add_err_msg(err, log_file);
            process::exit(1);
        });

        let tasks: Vec<Task> = serde_json::from_str(&contents).unwrap_or_else(|err| {
            Self::add_err_msg(Error::from(err), log_file);
            process::exit(1);
        });

        for t in tasks {

            if &t.id == task_id {
                task = Some(t);
                break;
            }

        }

        task
    }

    fn is_acc(task_id: &String, file_name: &String, log_file: &mut File) -> Done {
        let mut is_done: Done = Done::No;
        let mut file: File = File::open(file_name).unwrap_or_else(|err| {
            Self::add_err_msg(err, log_file);
            process::exit(1);
        });
        let mut content: String = String::new();
        file.read_to_string(&mut content).unwrap_or_else(|err| {
            Self::add_err_msg(err, log_file);
            process::exit(1);
        });
        let mut contents: Vec<Task> = serde_json::from_str(&content).unwrap_or_else(|err| {
            Self::add_err_msg(Error::from(err), log_file);
            process::exit(1);
        });

        file = OpenOptions::new().write(true).truncate(true).open(file_name).unwrap_or_else(|err| {
            Self::add_err_msg(err, log_file);
            process::exit(1);
        });

        for t in &mut contents {

            if task_id == &t.id {

                if t.accomplished {
                    is_done = Done::AlreadyDone;
                    break;
                }

                t.accomplished = true;
                is_done = Done::Yes;
                break;
            }

        }

        serde_json::to_writer_pretty(file, &contents).unwrap_or_else(|err| {
            Self::add_err_msg(Error::from(err), log_file);
            process::exit(1);
        });

        is_done
    }

    fn merge(all_task: &mut Vec<Task>, low: usize, high: usize, mid: usize) {
        let len_one: usize = mid - low + 1;
        let len_two: usize = high - mid;

        let mut arr_one: VecDeque<Task> = VecDeque::with_capacity(len_one);
        let mut arr_two: VecDeque<Task> = VecDeque::with_capacity(len_two);

        for i in 0..len_one {
            arr_one.push_back(all_task[low + i].clone());
        }

        for j in 0..len_two {
            arr_two.push_back(all_task[mid + j + 1].clone());
        }

        let mut k: usize = low;

        while !arr_one.is_empty() && !arr_two.is_empty() {
            let arr_one_ele: Task = arr_one.pop_front().unwrap();
            let arr_two_ele: Task = arr_two.pop_front().unwrap();

            if arr_one_ele.priority.val_to_prio() > arr_two_ele.priority.val_to_prio() {
                all_task[k] = arr_one_ele;
                arr_two.push_front(arr_two_ele);
            } else {
                all_task[k] = arr_two_ele;
                arr_one.push_front(arr_one_ele);
            }

            k += 1;
        }

        while !arr_one.is_empty() {
            all_task[k] = arr_one.pop_front().unwrap();
            k += 1;
        }

        while !arr_two.is_empty() {
            all_task[k] = arr_two.pop_front().unwrap();
            k += 1;
        }
    }

    fn sort(all_task: &mut Vec<Task>, low: usize, high: usize) {

        if low >= high {
            return;
        }

        let mid: usize = low + (high - low) / 2;
        Self::sort(all_task, low, mid);
        Self::sort(all_task, mid + 1, high);
        Self::merge(all_task, low, high, mid);
    }

    fn get_accor_to_prio(file_name: &String, log_file: &mut File) -> Vec<Task> {
        let mut file: File = File::open(file_name).unwrap_or_else(|err| {
            Self::add_err_msg(err, log_file);
            process::exit(1);
        });

        let mut contents: String = String::new();
        file.read_to_string(&mut contents).unwrap_or_else(|err| {
            Self::add_err_msg(err, log_file);
            process::exit(1);
        });
        let mut all_tasks: Vec<Task> = serde_json::from_str(&contents).unwrap_or_else(|err| {
            Self::add_err_msg(Error::from(err), log_file);
            process::exit(1);
        });
        let len: usize = all_tasks.len();
        Self::sort(&mut all_tasks, 0, len - 1);

        all_tasks
    }

    fn show_task(&self) {
        println!("\nid: {},\ncontent: {},\npriority: {},\naccomplished: {}\n",
                 &self.id, &self.content, Self::prio_is(&self.priority), self.accomplished);
    }
}

fn create_file(file_name: &String, log_file: &mut File) -> Done {
    let mut file: File = OpenOptions::new().create(true).write(true).open(file_name).unwrap_or_else(|err| {
        Task::add_err_msg(err, log_file);
        process::exit(1);
    });

    write!(file, "[]").unwrap();

    Done::Yes
}

fn clean_file(file_name: &String, log_file: &mut File) {
    let mut file: File = OpenOptions::new().write(true).truncate(true).open(file_name).unwrap_or_else(|err| {
        Task::add_err_msg(err, log_file);
        process::exit(1);
    });

    file.write("[]".as_bytes()).unwrap_or_else(|err| {
        Task::add_err_msg(err, log_file);
        process::exit(1);
    });

    println!("Done!");
}

fn main() {
    let mut log_file: File = OpenOptions::new().create(true).append(true).open("log.txt").unwrap_or_else(|err| {
        println!("Log file cannot access with error: {}", err);
        process::exit(1)
    });
    let o_args: Args = env::args();
    let args: Vec<String> = o_args.collect();

    if args.len() != 3 {
        println!("Required arguments is 3 got {}", args.len());
        return;
    }

    if args[2] == "-c" {
        create_file(&args[1], &mut log_file);
    } else if args[2] == "-a" {
        Task::new().add_task(&mut log_file, &args[1]);
    } else if args[2] == "-g" {
        print!("Enter the the ID: ");
        io::stdout().flush().expect("remove new line try again");

        let mut task_id: String = String::new();
        io::stdin().read_line(&mut task_id).expect("cannot read from the line try again");
        task_id = task_id.trim().parse().unwrap();

        let task: Option<Task> = Task::retrieve_task(&task_id, &args[1], &mut log_file);

        if let Some(t) = task {
            t.show_task();
        } else {
            println!("There is no such task with the id: {}", task_id)
        }

    } else if args[2] == "-u" {
        print!("Give the task Id: ");
        io::stdout().flush().expect("cannot remove new line");

        let mut task_id: String = String::new();
        io::stdin().read_line(&mut task_id).expect("cannot read from the line try again");
        task_id = task_id.trim().parse().unwrap();

        let task: Done = Task::is_acc(&task_id, &args[1], &mut log_file);

        match task {
            Done::Yes => { println!("Successfully task is done!"); }
            Done::No => { println!("Unfortunately no such task is present"); }
            Done::AlreadyDone => { println!("Unfortunately is already done"); }
        }

    } else if args[2] == "-p" {
        let tasks: Vec<Task> = Task::get_accor_to_prio(&args[1], &mut log_file);

        for t in tasks {
            t.show_task();
        }

    } else if args[2] == "-d" {
        clean_file(&args[1], &mut log_file);
    } else {
        println!("Err: There are only \n\t-c = create\n\t-a = append\n\t-g = get particular task with its id\n\t-u = update the task as accomplished\n\t-p = get data as arranged from high to low priority\n\t-d = clear all content from file");
    }
}
