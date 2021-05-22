// Add a semicolon after the macro. Just a syntax error. 

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
