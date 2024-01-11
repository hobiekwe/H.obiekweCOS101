use std::io;
fn main() {
    println!("Which laptop do you want?");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid input");

    println!("How many {} do you want?", input1);
    
    

}

