pub fn is_armstrong_number(num: u32) -> bool {
    let s = num.to_string();
    let exp = s.chars().count() as u32;

    let armstrong_calc: u32 = s.chars().map(|c| {
       let digit: u32 = c.to_digit(10).unwrap();
       digit.pow(exp)
    }).sum();
    num == armstrong_calc
}
