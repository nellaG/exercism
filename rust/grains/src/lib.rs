
pub fn square(s: u32) -> u64 {
    if 1 > s || s > 64 {
        panic!("Square must be between 1 and 64");
    } else {
        let x: u64 = 2;
        x.pow(s - 1)
    }
}

pub fn total() -> u64 {
    let mut sum: u64 = 0;
    let x: u64 = 2;
    for i in 1..65 {
        sum += x.pow(i - 1);
    }
    sum
}
