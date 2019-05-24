pub fn raindrops(n: u32) -> String {
    let pling_plang_plong = format!("{}{}{}", pling(n), plang(n), plong(n));
    if pling_plang_plong.is_empty() { n.to_string() } else { pling_plang_plong }
}

fn pling(n: u32) -> String {
    replace_if_factor(n, 3, "Pling")
}

fn plang(n: u32) -> String {
    replace_if_factor(n, 5, "Plang")
}

fn plong(n: u32) -> String {
    replace_if_factor(n, 7, "Plong")
}

fn replace_if_factor(n: u32, factor: u32, replacement_text: &str) -> String {
    if n % factor == 0 { replacement_text.to_string() } else { "".to_string() }
}
