use std::io::{self, Write};

struct Dimension {
    length: u32,
    breath: u32
}

fn cal_area(dim: &Dimension) -> u32 {
    dim.length * dim.breath
}

fn inp_details(length: u32, breath: u32) -> Dimension {
    Dimension {
        length: length,
        breath: breath
    }
}

impl Dimension {
    
    fn area(&self) -> u32 {
        self.length * self.breath
    }

    fn can_hold(&self, other: &Dimension) -> bool {
        self.length > other.length && self.breath > other.breath
    }

    fn square(size: u32) -> Dimension {
        Dimension {
            length: size,
            breath: size
        }
    }

}

fn main() {
    print!("Enter the length of the shape: ");
    io::stdout().flush().expect("Cannot flush new_line");
    
    let mut len: String = String::new();
    io::stdin().read_line(&mut len).expect("Cannot readline");
    
    let len: u32 = match len.trim().parse() {
        Ok(l) => l,
        Err(_) => 0
    };

    print!("Enter the breath of the shape: ");
    io::stdout().flush().expect("Cannot flush new_line");

    let mut bre: String = String::new();
    io::stdin().read_line(&mut bre).expect("Cannot readline");

    let bre: u32 = match bre.trim().parse() {
        Ok(l) => l,
        Err(_) => 0
    };

    let dim_one: Dimension = inp_details(len, bre);
    let area: u32 = cal_area(&dim_one);

    println!("The area is {} for length {} and breath {}", area, len, bre);

    let dim_two: Dimension = inp_details(5, 4);
    let area_two: u32 = dim_two.area();
    let can_hold: bool = dim_two.can_hold(&dim_one);

    let dim_sq: Dimension = Dimension::square(5);
    let val_sq: u32 = dim_sq.area();

    println!("The Square value: {}", val_sq);
    println!("Second The area is {} for length {} and breath {} can it hold {}", area_two, len, bre, can_hold);
}
