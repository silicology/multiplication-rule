fn calculate_series() -> f64 {
    let mut product = 1.0;
    for i in 0..100 {
        product *= (594 - i) as f64 / (600 - i) as f64;
    }
    product
}

fn main() {
    let result = calculate_series();
    println!("The result of the series is: {:.10}", result);
}
