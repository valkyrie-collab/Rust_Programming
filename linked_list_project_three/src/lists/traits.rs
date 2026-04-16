pub trait ListOp {
   fn append(&mut self, value: i32);
   fn show_list(&mut self);
   fn new() -> Self;
}