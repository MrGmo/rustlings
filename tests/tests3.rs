// Add is_even within assert! and add even values. Make sure the second function is followed by false because it should be odd. 

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(2));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(is_even(4), false);
    }
}
