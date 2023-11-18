use std::io;

fn main() {
    //loop system
    let number_researchers = 500;
    let mut researchers_counters = 0;

    while researchers_counters < number_researchers {
    let mut input_name = String::new();
    let mut input_numberofpapers = String::new();
    
//researcher's information
        println!("Enter name of researcher");
        io::stdin().read_line(&mut input_name).expect("Failed to read input");

        println!("Enter the number of papers published");
        io::stdin().read_line(&mut input_numberofpapers).expect("Failed to read input");
        let _numberofpapers: i32=input_numberofpapers.trim().parse().expect("Input is not a number");

        if (3..=5).contains(&_numberofpapers) {
    
        }    let incentive: i32 = 500_000;
            println! ("{} printed {} numbers of paper so your incentive is {}", input_name, _numberofpapers,incentive);
        
       if (5..10).contains(&_numberofpapers) {
       let incentive: i32 = 800_000;
       println! ("{} printed {} numbers of paper so your incentive is {}", input_name,_numberofpapers,incentive);
       }

       if _numberofpapers <= 3 {
           let incentive: i32 = 1_000_000;
           println! ("{} printed {} numbers of paper so your incentive is {}", input_name,_numberofpapers,incentive);
       }

       if _numberofpapers >= 10 {
           let incentive: i32 = 100_000;
           println! ("{} printed {} numbers of paper so your incentive is {}", input_name,_numberofpapers,incentive);
       }
    }

    }
    
