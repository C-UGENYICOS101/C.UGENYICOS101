fn main() {
    let a1:f64 = 450_000; // Toshiba
    let a2:f64 = 1_500_000; // Max
    let a3:f64 = 750_000; // HP
    let a4:f64 = 2_850_000; // Dell
    let a5:f64 = 250_000; //Acer
    
    // Quantities
    let q1 = 2.0;
    let q2 = 1.0;
    let q3 = 3.0;
    let q4 = 3.0;
    let q5 = 1.0;

    // Total Sales During Recession
    let t1 = a1*q1;
    let t2 = a2*q2;
    let t3 = a3*q3;
    let t4 = a4*q4;
    let t5 = a5*q5;

    // Total Amount 
    let sum = t1 + t2 + t3 + t4 + t5;

    // Average Sales
    let average = sum / 5.0;

    println!("Total sales during recession is {}", sum);
    println!("Average sales during recession is {}", average);
}
