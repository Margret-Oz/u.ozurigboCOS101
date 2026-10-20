use std::io::Write;

fn main() {
    let breweries = "Nigerian Breweries\n";

    let mut file = std::fs::File::create("Nigerian_Breweries.csv").expect("create failed");
    file.write_all("These are the Nigerian Breweries\n".as_bytes()).expect("write failed");
    file.write_all(breweries.as_bytes()).expect("write failed");
    println!("\nNigerian Breweries written to file.");

    let header = "\nLager,     Stout,     Non-Alcolholic";
    file.write_all(header.as_bytes()).expect("write failed");
    println!("{}", header);

    let line1 = "\n33 Export,     Legend,     Maltina";
    file.write_all(line1.as_bytes()).expect("write failed");
    println!("{}", line1);

    let line2 = "\nDesperados,     Turbo King,     Amstel Malta";
    file.write_all(line2.as_bytes()).expect("write failed");
    println!("{}", line2);

    let line3 = "\nGoldberg,     Willams,          Malta Gold";
    file.write_all(line3.as_bytes()).expect("write failed");
    println!("{}", line3);

    let line4 = "\nGulder,        ,            Fayrouz";
    file.write_all(line4.as_bytes()).expect("write failed");
    println!("{}", line4);

    let line5 = "\nHeineken,     ,             ,";
    file.write_all(line5.as_bytes()).expect("write failed");
    println!("{}", line5);



    println!("Thank you!")



}
