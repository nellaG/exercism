pub fn verse(n: i32) -> String {

    if n == 0 {
        "No more bottles of beer on the wall, no more bottles of beer.\n\
        Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    } else if n == 1 {
        "1 bottle of beer on the wall, 1 bottle of beer.\n\
        Take it down and pass it around, no more bottles of beer on the wall.\n".to_string()
    } else {
        let mut plural = "";
        if n - 1 > 1 {
            plural = "s";
        }
        format!("{init} bottles of beer on the wall, {init} bottles of beer.\n\
            Take one down and pass it around, {rest} bottle{plural} of beer on the wall.\n",
            init=n, rest=n-1, plural=plural)
    }

}

pub fn sing(start: i32, end: i32) -> String {
    let mut out = "".to_string();
    for index in (end..start + 1).rev() {
         if index == 0 {
             out.push_str("No more bottles of beer on the wall, no more bottles of beer.\n\
             Go to the store and buy some more, 99 bottles of beer on the wall.\n");
         } else if index == 1 {
             out.push_str("1 bottle of beer on the wall, 1 bottle of beer.\n\
             Take it down and pass it around, no more bottles of beer on the wall.\n");
         } else {
             let mut plural = "";
             if index - 1 > 1 {
                 plural = "s";
             }
             let fmt = format!("{start} bottles of beer on the wall, {start} bottles of beer.\n\
             Take one down and pass it around, {rest} bottle{plural} of beer on the wall.\n\
             ", start=index, rest=index-1, plural=plural);
             out.push_str(&fmt);
         }
         if (index - end) > 0 {
             out.push_str("\n");
         }
    }
    out
}
