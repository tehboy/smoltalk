use smol::eval;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn eval_in_browser(code: &str) -> String {
    eval(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = eval_in_browser("foo");
        assert_eq!(result, "foo");
    }
}
