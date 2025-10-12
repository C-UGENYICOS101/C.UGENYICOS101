fn main() {
    let p:f64 = 510_000;
    let r:f64 = 5;
    let n:f64 = 4;

    // Value after depreciation
    let a = p * ( 1.0 - (r / 100.0)).powf(n); 
    println!("Amount after {} years is {}", n, a);

    // Depreciation Amount 
    let ci = a - p;
    println!("Total depreciation after {} years is {}", n, depreciation);
}
