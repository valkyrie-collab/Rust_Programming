use std::process;

const N_STACK: usize = 1000;
const S_STACK: usize = 1000;
// const BUFF_LEN: usize = 100;

// static mut BUFF_ARR: [char; BUFF_LEN] = ['.'; BUFF_LEN];
// static mut BUFF_TOP: i32 = -1;

static mut NSTACK_ARR: [i32; N_STACK] = [-1; N_STACK];
static mut N_STACK_TOP: i32 = -1;

static mut SSTACK_ARR: [char; S_STACK] = ['.'; S_STACK];
static mut S_STACK_TOP: i32 = -1;

// fn unget_ch(c: char) {

//     unsafe {
//         BUFF_TOP += 1;

//         if BUFF_TOP as usize >= BUFF_LEN {
//             println!("BUFFER Overflow");
//         } else {
//             BUFF_ARR[BUFF_TOP as usize] = c;
//         }

//     }

// }

// fn get_ch() -> char {

//     unsafe {
//         BUFF_TOP -= 1;

//         if BUFF_TOP > -1 {
//             return BUFF_ARR[BUFF_TOP as usize];
//         } else {
//             println!("no chars")
//         }

//     }

//     '.'
// }

fn push_number(num: i32) {
    
    unsafe {

        if (N_STACK_TOP + 1) as usize >= N_STACK {
            println!("Stack overflow");
            return;
        } 
        
        N_STACK_TOP += 1;
        NSTACK_ARR[N_STACK_TOP as usize] = num;
        
        // println!("num {}", num);
        // println!("top {}", top);
    }

}

fn pop_number() -> i32 {
    let old_top: i32;
    
    unsafe {

        if N_STACK_TOP < 0 {
            println!("Stack underflow");
            return -1;
        }

        old_top = N_STACK_TOP;
        N_STACK_TOP -= 1;
        NSTACK_ARR[old_top as usize]
    } 

}

fn push_char(alpha: char) {

    unsafe {

        if (S_STACK_TOP + 1) as usize >= S_STACK {
            println!("char Stack overflow");
        } 
        
        S_STACK_TOP += 1;
        SSTACK_ARR[S_STACK_TOP as usize] = alpha

    }

}

fn pop_char() -> char {
    let old_top: i32;

    unsafe {

        if S_STACK_TOP < 0 {
            println!("char stack underflow");
            return 'a'
        }

        old_top = S_STACK_TOP;
        S_STACK_TOP -= 1;
        SSTACK_ARR[old_top as usize]
    }

}

fn calculate(symbol: char) {

    match symbol {
        '+' => {
            push_number(pop_number() + pop_number());
        }

        '-' => {
            let f_n: i32 = pop_number();
            push_number(pop_number() - f_n);
        }

        '*' => {
            push_number(pop_number() * pop_number());
        }

        '/' => {
            let f_n: i32 = pop_number();

            if f_n == 0 {
                println!("Err 0 cannot be divisor");
                process::exit(1);
            }

            push_number(pop_number() / f_n);
        }
        _ => {
            println!("The symbol is not expected");
        }
    }
    
}

fn separate_chars(eqn: &String) {
    let mut sum: i32 = 0;
    let mut prev_sym: char = '.';
    let mut is_num: bool = false;
    // let prio: [i32; 6] = [3, 2, 0, 1, 0, 4];

    for ch in eqn.chars() {

        if ch.is_digit(10) {
            //here we are joining the individual digit and marking them as true aka is_num = true
            sum = sum * 10 + (ch as i32 - 48);
            is_num = true;
        } else {

            //here we are checking if the current else if a number if it is then 
            //putting it in the number stack NSTACK_ARR which as size of NSTACK with N_STACK_TOP as its top
            //and making the is_num false and sum 0 because the current number is taken 
            if is_num {
                push_number(sum);
                sum = 0;
                is_num = false;
            }

            //here we are checking if the current char is first bracket; NOTE: "[]" and "{}" logic is not implemented
            //if it is then push it in SSTACK_ARR which has a size of STACK with S_STACK_TOP as its top and change
            //the prev_sym to the first '('
            if ch == '(' {
                push_char(ch);
                prev_sym = '(';
                //if the current is not first bracket then check if they are arithmatic sym
                //if it is then go to the if condition below;
            } else if ch == '+' || ch == '-' || ch == '*' || ch == '/' {

                //if the previous symbol is first bracket then push the current symbol to the SSTACK_ARR by eye
                //closing and change the previous symbol with current arithmatic symbol;
                if prev_sym == '(' {
                    push_char(ch);
                    prev_sym = ch;
                    //if the previous symbol is division and the current symbol is smaller than division
                    //then do the division first; because we know that division priority is highest than other
                    //symbols
                } else if prev_sym == '/' && (ch == '-' || ch == '+' || ch == '*') {
                    calculate(prev_sym);
                    //we are poping the current symbol "which is division" because its operation is over and
                    //we have no use for this operator
                    pop_char();
                    //and push the new symbol be it may be - + *
                    push_char(ch);
                    //and replace the current current previous with new symbol
                    prev_sym = ch;
                } else if prev_sym == '*' && (ch == '-' || ch == '+') {
                    //same as division but it is multiply
                    calculate(prev_sym);
                    pop_char();
                    push_char(ch);
                    prev_sym = ch;
                } else if prev_sym == '+' && ch == '-' {
                    //same as division but it is plus
                    calculate(prev_sym);
                    pop_char();
                    push_char(ch);
                    println!("prev sym: {} ch: {}", prev_sym, ch);
                    prev_sym = ch;
                } else {
                    //here it little different as we know lowest priority cannot be done before the highest 
                    //priority is over so if we find any type where the previous symbol priority is low than
                    //the current priority then current symbol will just be pushed in the SSTACK_ARR 
                    push_char(ch);
                    //And the previous symbol will be changed to the current symbol
                    prev_sym = ch;
                } 
                
            } else if ch == ')' {
                //after all hassel when we will get the closing bracket we will start calculation by poping
                //every digit and symbol from NSTACK_ARR and SSTACK_ARR untill first opening bracket is reached
                //or is the stack is underflow by using a while loop
                let mut pop_ch: char = pop_char();
                
                while pop_ch != '(' && pop_ch != '.' {
                    calculate(pop_ch);
                    pop_ch = pop_char();
                }

            }


        }

    }

}

fn main() {
    let eqn: String = String::from("(12 + 3 / (1 * 2))");
    separate_chars(&eqn);
    let val: i32 = pop_number();
    println!("value of eqn: {}", val);
}