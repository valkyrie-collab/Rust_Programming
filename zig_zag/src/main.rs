//lets create 2 array of given size straight_arr bent_arr
//straight_arr will take straight chars and bent with take bend chars
//and will be refreshed later on

use std::str::Chars;

struct Solution {
    str: String
}

impl Solution {
    fn new(s: String) -> Self {
        Solution { str: s }
    }

    fn zig_zag(self, num_rows: i32) -> String {
        if num_rows == 1 {
            return self.str;
        }

        let len: usize = self.str.len();
        let mut res: String = String::with_capacity(len);
        let chs: Vec<char> = self.str.chars().collect();

        for i in 0..num_rows {
            let incr: i32 = (num_rows - 1) * 2;
            let mut k: i32 = i;

            while k < len as i32 {
                res.push(chs[k as usize]);

                if i > 0 && i < (num_rows - 1) && (k + incr - 2 * i) < len as i32 {
                    res.push(chs[(k + incr - 2 * i) as usize])
                }

                k += incr;
            }

        }

        res
    }
}

fn main() {
    let sol: Solution = Solution::new(String::from("PAYPALISHIRING"));
    let str: String = sol.zig_zag(3);

    println!("{}", str);
}
