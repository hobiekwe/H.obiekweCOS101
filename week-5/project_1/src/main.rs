use std::io;

fn main() {
    println!("Enter the value of 'a':");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read line");
    let a: f64 = a.trim().parse().expect("Invalid input");

    println!("Enter the value of 'b':");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read line");
    let b: f64 = b.trim().parse().expect("Invalid input");

    println!("Enter the value of 'c':");
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Failed to read line");
    let c: f64 = c.trim().parse().expect("Invalid input");

    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: Root1 = {:.2}, Root2 = {:.2}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("One real root: Root = {:.2}", root);
    } else {
        println!("No real roots as discriminant is negative.");
    }
}
