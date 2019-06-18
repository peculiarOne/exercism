const QUESTION: &str = "Sure.";
const YELL: &str = "Whoa, chill out!";
const YELL_QUESTION: &str = "Calm down, I know what I'm doing!";
const SILENT: &str = "Fine. Be that way!";
const CATCH_ALL: &str = "Whatever.";

pub fn reply(message: &str) -> &str {

    let trimmed = message.trim();
    // is_shouted = true if all alphabetic characters are uppercase
    let is_shouted = {
        let alpha_only: String = trimmed.chars().filter(|c| c.is_alphabetic()).collect();
        // .all() returns true for an empty iter, so need an explicit check
        !alpha_only.is_empty() && alpha_only.chars().all(|c| c.is_uppercase())
    };

    let last_char = trimmed.chars().rev().next();
    match last_char {
        Some('?') => if is_shouted { YELL_QUESTION } else { QUESTION },
        Some(_) => if is_shouted { YELL } else { CATCH_ALL },
        None => SILENT, // no last character == empty message
    }
}