pub fn reverse(input: &str) -> String {
    let mut reversed = String::new();
    for c in input.chars().rev() {
        reversed.push(c);
    };
    reversed
    // unimplemented!("Write a function to reverse {}", input);
}
