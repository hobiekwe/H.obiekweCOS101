fn main() {
    let studentname = vec! ["Oluchi Mordi", "Adams Aliyu", "Shanla Bolade", "Adekunle Gold", "Bianca Edemoh"];
    let matricnumber = vec! ["ACC1O211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let department = vec! ["Accounting", "Economics", "Computer", "Electrical", "MechanicL"];
    let level = vec! ["300", "100", "200", "200", "100"];

    print! ("\nPAU SMS:\n");
    for i in 0..level.len(){
        print! ("{} with maticulation number {} in {} department also in {} level\n", studentname[i], matricnumber[i], department[i], level[i]);
    }
}