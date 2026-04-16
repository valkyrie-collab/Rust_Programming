pub mod single;
mod node;
pub mod double;
pub mod traits;

use single::SingleList;
use double::DoubleList;
use traits::ListOp;

pub fn new_single_list() -> SingleList {
    SingleList::new()
}

pub fn new_double_list() -> DoubleList {
    DoubleList::new()
}
