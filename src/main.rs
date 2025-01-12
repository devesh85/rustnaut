use crate::exercises::string_manipulation::reverse;
pub mod exercises;
use crate::rust_book::ref_and_borrow::borrow;
pub mod rust_book;

fn main() {
    let s = "Devesh Kishore";
    let rev_s = reverse(s);
    print!("{rev_s}\n");
    borrow();
}
