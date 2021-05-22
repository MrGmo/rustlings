// Implement a simple if/else statement within your function with the price

fn calculate_apple_price(num: i32) -> i32 {
    if num > 40 {
        num
    }else{
        num*2
    }
}

#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
}
