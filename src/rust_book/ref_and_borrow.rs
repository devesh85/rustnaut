pub  fn borrow(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}");
}

pub fn change_string(some_string: &mut String){
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize{
    s.len()
}
