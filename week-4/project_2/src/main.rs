use std::io;

fn main() {
    let mut experience_input = String::new();
    let mut age_input = String::new();

    println!("Is the employee experienced? (yes/no):");
    io::stdin()
        .read_line(&mut experience_input)
        .expect("Failed to read input");

    println!("Enter the employee's age:");
    io::stdin()
        .read_line(&mut age_input)
        .expect("Failed to read input");

    let experienced = experience_input.trim().to_lowercase();
    let age: i32 = age_input
        .trim()
        .parse()
        .expect("Please enter a valid age");

    if experienced == "yes" {
        if age >= 40 {
            println!("Annual incentive: N1,560,000");
        } else if age >= 30 && age <= 39 {
            println!("Annual incentive: N1,480,000");
        } else if age < 28 {
            println!("Annual incentive: N1,300,000");
        } else {
            println!("No incentive category was provided for ages 28 and 29.");
        }
    } else if experienced == "no" {
        println!("Annual incentive: N100,000");
    } else {
        println!("Please enter either yes or no for experience.");
    }
}