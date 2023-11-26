#[no_mangle]
pub fn factorial(n: i64) -> i64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
