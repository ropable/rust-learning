use std::i32;

pub fn is_leap_year(year: i32) -> bool {
    if year % 4 == 0 {
        if year % 400 == 0 {
            return true;
        }
        if year % 100 == 0 {
            return false;
        }
    }
    year % 4 == 0
}
