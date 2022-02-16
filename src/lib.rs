#![allow(non_snake_case)]
#[cfg(test)]

mod core;


    #[test]
    fn itWorks() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn HelloWorld() {
        assert_eq!("Hello from SetString!", core::sayHello());
    }
