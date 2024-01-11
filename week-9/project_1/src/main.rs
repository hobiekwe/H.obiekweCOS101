use std::io::Write;

fn main() {

    let announce = "Welcome to Nigeria Brewery Limited\n";
    let dept = "Our Rich portfolio of drinks";
    let drinks = vec! ["lager", "stout", "non_alcoholic"]; 

    let lager = ["33 export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout =  ["Legend", "Turbo", "Williams"];
    let non_alcoholic =  ["AmstelMalta", "Fayrouz"];
    
  
    
    

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Welcome to Rust Programming\n".as_bytes()).expect("write failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");
    println! ("categories successfullly created");
    


    

    
   



    
}