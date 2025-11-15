pub fn eval(code: &str) -> String {
    format!("{}", code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(eval("foo"), "foo");
    }
}
