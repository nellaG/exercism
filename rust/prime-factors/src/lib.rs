pub fn factors(input: u64) -> Vec<u64> {
    let mut num = input;
    let mut _factors: Vec<u64> = Vec::new();
    let mut factor = 2;
    while num > 1 {
        if num % factor == 0 {
            _factors.push(factor);
            num /= factor;
            factor = 2;
        } else {
            factor += 1;
        }
    }
    _factors
}
