struct Usr {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn generate_user(name: String, email: String, s_cnt: u64, active: bool) -> Usr {
    Usr {
        username: name,
        email: email,
        sign_in_count: s_cnt,
        active: active
    }
}

fn main() {
    let name: String = String::from("rajarshi");
    let email: String = String::from("rajarshi@gmail.com");
    let s_cnt: u64 = 0;
    let active: bool = true;

    let user: Usr = generate_user(name, email, s_cnt, active);
    let user_two: Usr = Usr {
        username: String::from("valkyrie"),
        email: String::from("valkyrie@gmail.com"),
        sign_in_count: 1,
        active: false
    };

    let name: String = String::from("rajarshi");
    let email: String = String::from("rajarshi@gmail.com");
    let mut user_third: Usr = generate_user(name, email, s_cnt, active);

    println!("Value of user one: name {}, email {}, sign {}, act {}", user.username, user.email, user.sign_in_count, user.active);
    println!("Value of user two: name {}, email {}, sign {}, act {}", user_two.username, user_two.email, user_two.sign_in_count, user_two.active);
    println!("Value of user third: name {}, email {}, sign {}, act {}", user_third.username, user_third.email, user_third.sign_in_count, user_third.active);

    user_third.username = String::from("valkyrie");
    println!("New value of user third: name {}, email {}, sign {}, act {}", user_third.username, user_third.email, user_third.sign_in_count, user_third.active);
}
