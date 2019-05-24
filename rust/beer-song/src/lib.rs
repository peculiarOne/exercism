pub fn verse(n: i32) -> String {
    let bottle_bottles = |num: i32| if num == 1 { "bottle" } else { "bottles" };
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\n\
              Go to the store and buy some more, 99 bottles of beer on the wall.\n"
            .to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\n\
              Take it down and pass it around, no more bottles of beer on the wall.\n"
            .to_string(),
        _ => format!(
            "{a} bottles of beer on the wall, {a} bottles of beer.\n\
             Take one down and pass it around, {b} {bottles} of beer on the wall.\n",
            a = n,
            b = n - 1,
            bottles = bottle_bottles(n - 1)
        ),
    }
}

pub fn sing(start: i32, end: i32) -> String {
    (end..=start)
        .rev()
        .map(|n| verse(n))
        .collect::<Vec<_>>()
        .join("\n")
}
