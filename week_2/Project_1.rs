fn main() {
    let p:f64 = 520_000_000;
    let r:f64 = 10;
    let n:f64 = 5;

    // Compound Amount
    let a = p * ( 1.0 + (r / 100.0)).powf(n); 
    println!("Amount after {} years is {}", n, a);

    // Compound Interest
    let ci = a - p;
    println!("Compound Interest is {}", ci);
}
