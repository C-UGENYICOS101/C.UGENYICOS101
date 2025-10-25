// Rust program to find the roots of a quadratic equation

use std::io;

fn main() 
{
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter a value for a (not equal to 0): ");
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a: f32 =a.trim().parse().expect("Not a valid number");


    println!("Enter a value for b: ");
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b: f32 = b.trim().parse().expect("Not a valid number");

    println!("Enter a value for c: ");
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c: f32 = c.trim().parse().expect("Not a valid number");

    let d = b.powi(2)- 4.0 * a *c; // discriminant

    if d > 0.0 {
    let x1 = (-b + d.sqrt()) / (2.0 * a);
    let x2 = (-b - d.sqrt()) / (2.0 * a);
    println!("The roots of the equation are: {}, {}", x1, x2);
    } else if d == 0.0 {
        let x = -b / (2.0 * a);
        println!("The root of the equation is {}", x);
    } else if d < 0.0 {
        println!("The equation has no real roots")
    }
}