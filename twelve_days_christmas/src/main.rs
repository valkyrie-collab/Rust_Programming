fn main() {
    let arr: [&str; 12] = [
        "A partridge in a pear tree.", "Two turtle doves,", "Three French hens,",
        "Four calling birds,", "Five golden rings,", "Six geese a-laying,", 
        "Seven swans a-swimming,", "Eight maids a-milking,", "Nine ladies dancing,",
        "Ten lords a-leaping,", "Eleven pipers piping,", "Twelve drummers drumming,"
    ];
    let mut range: usize = 0;
    let mut temp: usize;

    while range < 12 {
        temp = range;
        let day: &str = match range {
            0 => "First",
            1 => "Second",
            2 => "Third",
            3 => "Fourth",
            4 => "Fifth",
            5 => "Sixth",
            6 => "Seventh",
            7 => "Eighth",
            8 => "Ninth",
            9 => "Tenth",
            10 => "Eleventh",
            11 => "Twelfth",
            _ => "invalid"
        };

        println!("On the {} day of Christmas my true love sent to me:", day);

        while temp > 0 {
            println!("{}", arr[temp]);
            temp -= 1;
        }

        println!("{}", arr[temp]);
        range += 1;
        println!("\n");
    }

}
