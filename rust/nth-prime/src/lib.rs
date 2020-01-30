pub fn nth(input: i32) -> Result<i64, &'static str> {
    let mut nth: i64 = 1;
    let mut index: i32 = 0;
    if input < 1 {
        return Result::Err("");
    } else
    if input == 1 {
        nth = 2;
    } else if input == 2 {
        nth = 3;
    } else {
        while index < input {
            nth += 1;
            index += prime(nth);
        }
    }
    Result::Ok(nth)
}

fn prime(input: i64) -> i32 {
    for i in 2..(input as f64).sqrt() as i64 + 1 {
        if input % i == 0 {
            return 0;
        }
    }
    return 1;
}