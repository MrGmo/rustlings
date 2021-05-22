// Replace return assert_eq! in both tests and include times_two as parameters; adding positive/negative versions of the same nums * 2

pub fn times_two(num: i32) -> i32 {
    num * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_twice_of_positive_numbers() {
        assert_eq!(times_two(4), 8);
    }

    #[test]
    fn returns_twice_of_negative_numbers() {
        assert_eq!(times_two(-4), -8);
    }
}
