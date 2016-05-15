pub fn square_of_sum(n: i32) -> i32 {
    let sum = (1..n+1).fold(0, |sum, x| sum + x);
    sum * sum
}

pub fn sum_of_squares(n: i32) -> i32 {
    (1..n+1).map(|n| n*n).fold(0, |sum, x| sum + x)
}

pub fn difference(n: i32) -> i32 {
    square_of_sum(n) - sum_of_squares(n)
}
