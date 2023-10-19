fn main(){
    let p:f64 = 450_000.00;
    let r:f64 = 1_500_000.00;
    let n:f64 = 750_000.00;
    let x:f64 = 2_850_000.00;
    let y:f64 = 250_000.00;

    //to find sum 

    let s = p + r + n + x + y;
    println!("Sum of the sales {}", s);

    //average 

    let a = s / 5.0;
    println!("Average of the sales {}", a);


}
