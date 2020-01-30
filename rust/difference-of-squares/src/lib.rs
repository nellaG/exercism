pub fn square_of_sum(n: usize) -> usize {
    let sum: usize = (1..n + 1).fold(0, |sum, x| sum + x).pow(2);
    sum
}

pub fn sum_of_squares(n: usize) -> usize {
    let sum: usize = (1..n + 1).fold(0, |sum, x| sum + x.pow(2));
    sum
}

pub fn difference(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}
