use std::io;

fn main() {

let number_voters = 150;
let mut voters_counter = 0;

while voters_counter < number_voters {
    let mut input_name = String::new();
    let mut input_email = String::new();
    let mut input_department = String::new();
    let mut input_stateoforigin = String::new();
    let mut input_level = String::new();
    let mut input_cgpa = String::new();
    let mut input_classrep = String::new();

//Candidates information

    println! ("Enter your name");
    io::stdin().read_line(&mut input_name).expect("Failed to read input");

    println! ("Enter your email");
    io::stdin().read_line(&mut input_email).expect("Failed to read input");

    println! ("Enter your department");
    io::stdin().read_line(&mut input_department).expect("Failed to read input");

    println! ("Enter your state of origin");
    io::stdin().read_line(&mut input_stateoforigin).expect("Failed to read input");

    println! ("Enter your level");
    io::stdin().read_line(&mut input_level).expect("Failed to read input");
    let _level: i32 = input_level.trim().parse().expect("Input is not a number");

    println! ("Enter your CGPA");
    io::stdin().read_line(&mut input_cgpa).expect("Failed to read input");
    let _cgpa: f32 = input_cgpa.trim().parse().expect("Input is not a number");

    println! {"Enter candidates_status 'class rep' or 'not a class rep' : "}
    io::stdin().read_line(&mut input_classrep).expect("Failed to read input");
    let candidates_status:String = input_classrep.trim().parse().expect("Invalid status");

    if _cgpa >= 4.0 && _level > 100 && candidates_status == "class rep" {
        println!{"You are eligible to vote {}!, ", input_name};
    }
    else {
        println!("You are not eligible to vote {}!", input_name);
    }

        println! ("Candidate's Information");
        println! ("Name: {}", input_name.trim());
        println! ("Email: {}", input_email.trim());
        println! ("Department: {}", input_department.trim());
        println! ("State of Origin: {}", input_stateoforigin.trim());
        println! ("Level: {}", input_level.trim());
        println! ("CGPA: {}", input_cgpa.trim());
        
    

}


}