pub fn reply(input: &'static str) -> &'static str {
    let saying = input.trim();
    if saying.is_empty() {
        "Fine. Be that way!"
    } else if saying.chars().all(|x| !x.is_alphabetic()) && saying.ends_with("?") {
        "Sure."
    } else if saying.chars().all(|x| !x.is_alphabetic()) {
        "Whatever."
    } else if saying.chars().filter(|x| x.is_alphabetic()).all(|x| !x.is_lowercase()) {
        "Whoa, chill out!"
    } else if saying.ends_with("?") {
        "Sure."
    } else {
        "Whatever."
    }
}