#![allow(non_snake_case)]
#[cfg(test)]

mod core;

mod tests {

    #[test]
    fn itWorks() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn HelloWorld() {
        assert_eq("Hello World!", SetString());
    }
}