use std::io;
use std::f64::consts::PI;

fn main() {
    loop {
        println!("Select the calculation you want to perform:");
        println!("1: Area of Trapezium");
        println!("2: Area of Rhombus");
        println!("3: Area of Parallelogram");
        println!("4: Area of Cube");
        println!("5: Volume of Cylinder");
        println!("Enter any other key to exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                let (base1, base2, height) = get_three_inputs();
                println!("Area of Trapezium: {}", area_of_trapezium(base1, base2, height));
            },
            "2" => {
                let (diagonal1, diagonal2) = get_two_inputs();
                println!("Area of Rhombus: {}", area_of_rhombus(diagonal1, diagonal2));
            },
            "3" => {
                let (base, altitude) = get_two_inputs();
                println!("Area of Parallelogram: {}", area_of_parallelogram(base, altitude));
            },
            "4" => {
                let side = get_single_input();
                println!("Area of Cube: {}", area_of_cube(side));
            },
            "5" => {
                let (radius, height) = get_two_inputs();
                println!("Volume of Cylinder: {}", volume_of_cylinder(radius, height));
            },
            _ => break,
        }
    }
}

fn get_single_input() -> f64 {
    println!("Enter the side length:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please type a number!")
}

fn get_two_inputs() -> (f64, f64) {
    println!("Enter the first value:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let first = input1.trim().parse().expect("Please type a number!");

    println!("Enter the second value:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let second = input2.trim().parse().expect("Please type a number!");

    (first, second)
}

fn get_three_inputs() -> (f64, f64, f64) {
    println!("Enter the first base:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let first = input1.trim().parse().expect("Please type a number!");

    println!("Enter the second base:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let second = input2.trim().parse().expect("Please type a number!");

    println!("Enter the height:");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read line");
    let third = input3.trim().parse().expect("Please type a number!");

    (first, second, third)
}

fn area_of_trapezium(base1: f64, base2: f64, height: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

fn area_of_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn area_of_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn area_of_cube(side: f64) -> f64 {
    6.0 * side.powi(2)
}

fn volume_of_cylinder(radius: f64, height: f64) -> f64 {
    PI * radius.powi(2) * height
}
