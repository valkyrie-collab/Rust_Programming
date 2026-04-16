use std::io;

static mut STACK: [i32; 10] = [0; 10];
static mut TOP: usize = 0;

fn usr_input() -> String {
    println!("Enter the equation below");
    let mut eqn: String = String::new();
    io::stdin().read_line(&mut eqn).expect("Read error");

    eqn
}

fn push(val: i32) {

    unsafe {

        if TOP >= 10 {
            println!("The stack is full...");
        } else {
            STACK[TOP] = val;
            TOP += 1;
        }

    }

}

fn pop() -> i32 {
    
    unsafe {

        if TOP <= 0 {
            println!("The is underflow...");
            return -1;
        } else {
            TOP -= 1;
            return STACK[TOP];
        }

    }

}

fn do_cal(sym: &str) {
    let obj: i32; 

    println!("The value of str: {}", sym);

    if sym.eq("+") || sym.eq("+\n") {
        push(pop() + pop());
    } else if sym.eq("*") || sym.eq("*\n") {
        push(pop() * pop());
    } else if sym.eq("-") || sym.eq("-\n") {
        obj = pop();
        push(pop() - obj);
    } else if sym.eq("/") || sym.eq("/\n") {
        obj = pop();
        push(pop() / obj);
    } else {
        println!("not a valid input");
    }
}

fn is_digit(str: &str) -> bool {
    
    for i in str.chars() {

        if i == '.' {
            continue;
        }

        if i >= '0' && i <= '9' {
            continue;
        } else {
            return false;
        }
    }

    true
}

fn get_digit(str: &str) -> i32 {
    str.trim().parse().expect("it is not a digit..")
}

fn get_top(str: &str) {

    for i in str.split(" ") {
        
        if is_digit(i) {
            push(get_digit(i));
        } else {
            do_cal(i);
        }

    }

}

fn main() {
    let mut eqn: String = usr_input();

    while !eqn.eq("exit") || !eqn.eq("stop") {
        get_top(&eqn);

        unsafe {
            TOP -= 1;
            println!("The summation of the eqn {} is {}", eqn, STACK[TOP])
        }

        eqn = usr_input();
    }

}
