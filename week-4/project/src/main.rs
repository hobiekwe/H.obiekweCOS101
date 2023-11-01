//Rust program to calculate speed of a car

use std::io;

fn main() {
    // Ask the user for input: distance in miles
    println!("Enter the distance traveled in miles: ");
    let mut distance = String::new();
    io::stdin().read_line(&mut distance).expect("Failed to read input");
    let d:f32= distance.trim().parse().expect("Invalid input. Please enter a number.");

    println!("Enter the time taken in hours");
    let mut time = String::new();
    io::stdin().read_line(&mut time).expect("Failed to read input");
    let t:f32 = time.trim().parse().expect("Invalid input. Please enter a number.");

    
    
  
    // Convert miles to kilometers (1 mile = 1.60934 kilometers)
    let distance_km = d * 1.60934;
    let speed_kmph = d/t;
    let speed_mph = distance_km/t;

    



    // Display the results
    println!("The speed of the car is: {} miles per hour", speed_mph);
    println!("The distance traveled in kilometers is: {} kilometers", distance_km);
    println!("The speed of the car is: {} kilometers per hour", speed_kmph);
}