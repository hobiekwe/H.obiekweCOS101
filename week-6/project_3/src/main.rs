//Rust program displaying multiplication
use std::io;
fn main(){
    let numbers = 15;
    let mut number_counter = 0; {

        println! {"Input number"};
        let mut input_number = String::new();
        io::stdin().read_line(&mut input_number).expect("Failed to read input");
        let n:i64 = input_number.trim().parse().expect("Not a valid number");

        let multiplication1: i64= n*1;
        let multiplication2: i64= n*2;
        let multiplication3: i64= n*3;
        let multiplication4: i64= n*4;
        let multiplication5: i64= n*5;
        let multiplication6: i64= n*6;
        let multiplication7: i64= n*7;
        let multiplication8: i64= n*8;
        let multiplication9: i64= n*9;
        let multiplication10: i64= n*10;
        let multiplication11: i64= n*11;
        let multiplication12: i64= n*12;
        let multiplication13: i64= n*13;
        let multiplication14: i64= n*14;
        let multiplication15: i64= n*15;


        println! ("Multiplication of 1 to n");
        println! ("{}", multiplication1);
        println! ("{}", multiplication2);
        println! ("{}", multiplication3);
        println! ("{}", multiplication4);
        println! ("{}", multiplication5);
        println! ("{}", multiplication6);
        println! ("{}", multiplication7);
        println! ("{}", multiplication8);
        println! ("{}", multiplication9);
        println! ("{}", multiplication10);
        println! ("{}", multiplication11);
        println! ("{}", multiplication12);
        println! ("{}", multiplication13);
        println! ("{}", multiplication14); 
        println! ("{}", multiplication15);

        
    }
   


}