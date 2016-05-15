pub fn is_leap_year(year: i32) -> bool {
    let is_divisable = |n| year % n == 0;
    is_divisable(4) && (!is_divisable(100) || is_divisable(400))
}
