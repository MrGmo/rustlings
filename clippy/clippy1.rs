// Add an error boundary to the floating numbers and use an inequality

fn main() {
    let x = 1.2331f64;
    let y = 1.2332f64;
    let error = 0.0001f64;
    if (x - y).abs() < error {
        println!("Success!");
    }
}
