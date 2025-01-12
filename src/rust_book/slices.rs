pub fn first_word_size(s: &String) -> usize{
    let bytes = s.as_bytes();

    for(i, item) in bytes.iter().enumerate(){
        if *item == b' ' {
            return i;
        }
    }

    s.len()
}
