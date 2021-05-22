// Parse the fromStr properly and extract what you need to complete the problem. 

use std::str::FromStr;

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

impl FromStr for Person {
    type Err = String;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        let mut iter = s.split(",");
        let name: String;
        let age: usize;

        let n = iter.next().unwrap();
        if n != "" {
            name = n.to_string();
        } else { return Err("Error".to_owned()) }

        if let Some(age_str) = iter.next(){
            match age_str.parse::<usize>() {
                Ok(a) => age = a,
                Err(_) => return Err("Error".to_owned()),
            }
        } else { return Err("Error".to_owned()); }

        Ok(Person{name, age})
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert!("".parse::<Person>().is_err());
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    #[should_panic]
    fn missing_age() {
        "John,".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_age() {
        "John,twenty".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_comma_and_age() {
        "John".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_name() {
        ",1".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_name_and_age() {
        ",".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_name_and_invalid_age() {
        ",one".parse::<Person>().unwrap();
    }
}
