extern crate chrono;
use chrono::*;

pub fn after(start_date: DateTime<UTC>) -> DateTime<UTC> {
    // This function accepts and returns a UTC DateTime.
    let x: i64 = 1000000000;
    let result = start_date + Duration::seconds(x);
    result
}
