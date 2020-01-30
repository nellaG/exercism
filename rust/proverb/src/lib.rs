pub fn build_proverb(list: Vec<&str>) -> String {

    let last = format!("And all for the want of a {horseshoe}nail.",
        horseshoe=(if list.len() > 2 { "horseshoe " } else { "" }));
    let mut out: Vec<String> = Vec::new();
    if list.is_empty() {
        String::new()
    } else {
        for i in 0..(list.len() - 1) {
            out.push(format!("For want of a {first} the {second} was lost.",
            first=list[i], second=list[i + 1]));
        }
        out.push(last);
        out.as_slice().join("\n")
    }
}
