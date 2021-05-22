// Include necessary functions within macro_rules! my_macro to concatenate strings.

#[macro_export]
macro_rules! my_macro {
    ($("world!"),*) => {
        "Hello world!"
    };
    ($("goodbye!"),*) => {
        "Hello goodbye!"
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_macro_world() {
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
}
