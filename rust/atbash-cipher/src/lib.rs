// implementation notes:
//  - strip punctuation (includes spaces)
//  - strip non-ascii
//  - preserve numbers
//  - all to lower case
//  - decode to a single continuous string
use std::collections::HashMap;

const ENCODABLE_ALPHAS: &str = "abcdefghijklmnopqrstuvwxyz";

fn create_lookup_table() -> HashMap<char, char> {
    let forward_chars = ENCODABLE_ALPHAS.chars();
    let backward_chars = ENCODABLE_ALPHAS.chars().rev();

    forward_chars.zip(backward_chars).collect()
}

fn apply_cipher(to_encode: &str, lookup_table: &HashMap<char, char>) -> String {
    to_encode
        .chars()
        .flat_map(|c| match c {
            x if x.is_digit(10) => Some(x),
            _ => (lookup_table.get(&c).map(|c| *c)),
        })
        .collect()
}

fn insert_spaces(unspaced: &str) -> String {
    unspaced
        .chars()
        .enumerate()
        .flat_map(|(index, ch)| {
            match index {
                i if i != 0 && i % 5 == 0 => Some(' '),
                _ => None,
            }
            .into_iter() // turn our Option<char> into an iterator
            .chain(std::iter::once(ch)) // append another iterator containing just the char at this index
        })
        .collect()
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let plain_lower = plain.to_lowercase();
    insert_spaces(&apply_cipher(&plain_lower, &create_lookup_table()))
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    apply_cipher(cipher, &create_lookup_table())
}
