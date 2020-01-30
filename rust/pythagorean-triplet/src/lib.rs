pub fn find() -> Option<u64> {
    for i in 1..(1000 / 3) {
        for j in (i + 1)..((1000 - i) / 2) {
            if i * i + j * j == (1000 - i - j) * (1000 - i - j) {
                return Some(i * j * (1000 - i - j));
            }
        }
    }
    None 
}