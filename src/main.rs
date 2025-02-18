use crate::exercises::string_manipulation::reverse;
pub mod exercises;
use crate::rust_book::ref_and_borrow::{borrow, change_string};
use crate::rust_book::slices::first_word_size;
pub mod rust_book;

fn main() {
    let s = "Devesh Kishore";
    let rev_s = reverse(s);
    print!("{rev_s}\n");
    borrow();
    let mut s = String::from("hello");
    change_string(&mut s);
    let x = String::from("Mera joota hai japani");
    let len = first_word_size(&x);
    print!("{len}\n");  
  
}
