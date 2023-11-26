use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn factorial(n: i64) -> i64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str); // import external window.alert function
}

#[wasm_bindgen]
pub fn factorial_alert(n: i64) {
    alert(format!("The factorial of {} is: {}", n, factorial(n)).as_str());
}
