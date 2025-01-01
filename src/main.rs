use crate::exercises::string_manipulation::reverse;
pub mod exercises;

fn main() {
    let s = "Devesh Kishore";
    let rev_s = reverse(s);
    print!("{rev_s}\n")
}
