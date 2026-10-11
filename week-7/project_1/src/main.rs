use std::io;

fn main() {
    println!("--- Shape Calculator ---");
    println!("1. Trapezium Area");
    println!("2. Rhombus Area");
    println!("3. Parallelogram Area");
    println!("4. Cube Surface Area");
    println!("5. Cylinder Volume");
    println!("Enter your choice (1-5):");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: u32 = choice.trim().parse().expect("Please enter a number");

    match choice {
        1 => println!("Result: {}", area_trapezium()),
        2 => println!("Result: {}", area_rhombus()),
        3 => println!("Result: {}", area_parallelogram()),
        4 => println!("Result: {}", area_cube()),
        5 => println!("Result: {}", volume_cylinder()),
        _ => println!("Invalid choice!"),
    }
}

fn get_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid number")
}

fn area_trapezium() -> f64 {
    let h = get_input("Enter height:");
    let b1 = get_input("Enter base1:");
    let b2 = get_input("Enter base2:");
    h / 2.0 * (b1 + b2)
}

fn area_rhombus() -> f64 {
    let d1 = get_input("Enter diagonal1:");
    let d2 = get_input("Enter diagonal2:");
    0.5 * d1 * d2
}

fn area_parallelogram() -> f64 {
    let b = get_input("Enter base:");
    let a = get_input("Enter altitude:");
    b * a
}

fn area_cube() -> f64 {
    let s = get_input("Enter side:");
    6.0 * s * s
}

fn volume_cylinder() -> f64 {
    let r = get_input("Enter radius:");
    let h = get_input("Enter height:");
    std::f64::consts::PI * r * r * h
}
