extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let reversed_graphemes: String = input.to_string().graphemes(true).rev().collect();
    reversed_graphemes
}
