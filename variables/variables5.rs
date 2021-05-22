// Change the data types by changing the scope and re-declare the variable

fn main() {
    let number = "3";
    println!("Spell a Number : {}", number);
    {
    let number = 3;
    println!("Number plus two is : {}", number + 2);
    }
}
