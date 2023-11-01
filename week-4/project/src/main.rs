//Rust program to calculatr]e speed of a car

use std::io;

fn main() {
    // Prompt the user for input
    println!("Enter the distance traveled in miles: ");
    let mut distance = String::new();
    io::stdin().read_line(&mut distance).expect("Failed to read input");

    println!("Enter the time taken in hours: ");
    let mut time = String::new();
    io::stdin().read_line(&mut time).expect("Failed to read input");

    // Parse input as f64 (floating-point number)
    let distance: f64 = distance.trim().parse().expect("Invalid input. Please enter a number.");
    let time: f64 = time.trim().parse().expect("Invalid input. Please enter a number.");

    // Calculate speed (distance / time)
    let speed = distance / time;

    // Display the result
    println!("The speed of the car is: {} miles per hour", speed);
}  