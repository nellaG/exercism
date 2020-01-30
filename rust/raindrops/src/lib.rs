pub fn raindrops(number: i32) -> String {
    let mut result = String::from("");
    let pling = number % 3 == 0;
    let plang = number % 5 == 0;
    let plong = number % 7 == 0;
    if !pling && !plang && !plong {
        return number.to_string();
    }
    match pling {
        true => result.push_str("Pling"),
        _ => (),
    }
    match plang {
        true => result.push_str("Plang"),
        _ => (),
    }
    match plong {
        true => result.push_str("Plong"),
        _ => (),
    }
    return result;
}