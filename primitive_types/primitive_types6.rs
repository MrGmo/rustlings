// Set second to second in tuple 

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    let second = numbers.1;

    assert_eq!(2, second,
        "This is not the 2nd number")
}
