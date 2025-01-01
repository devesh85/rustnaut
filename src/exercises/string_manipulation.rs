pub fn reverse(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut buffer: Vec<char> = Vec::new();
    for i in 0..len {
        buffer.push(chars[len - i - 1]);
    }

    buffer.into_iter().collect()
}
