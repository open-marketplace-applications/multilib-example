pub fn greet() -> String {
    format!("Hello World!")
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(greet(), "Hello World!");
    }
}
