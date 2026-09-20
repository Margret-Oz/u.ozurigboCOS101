use std::io;

fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();
    let mut input_c = String::new();

    println!("Enter value of a:");
    io::stdin()
        .read_line(&mut input_a)
        .expect("Failed to read input");
    let a: f64 = input_a
        .trim()
        .parse()
        .expect("Please enter a valid number");

    println!("Enter value of b:");
    io::stdin()
        .read_line(&mut input_b)
        .expect("Failed to read input");
    let b: f64 = input_b
        .trim()
        .parse()
        .expect("Please enter a valid number");

    println!("Enter value of c:");
    io::stdin()
        .read_line(&mut input_c)
        .expect("Failed to read input");
    let c: f64 = input_c
        .trim()
        .parse()
        .expect("Please enter a valid number");

    if a == 0.0 {
        println!("This is not a quadratic equation because a cannot be zero.");
    } else {
        let d = b * b - 4.0 * a * c;

        if d > 0.0 {
            let root1 = (-b + d.sqrt()) / (2.0 * a);
            let root2 = (-b - d.sqrt()) / (2.0 * a);

            println!("The equation has two distinct real roots:");
            println!("Root 1 = {}", root1);
            println!("Root 2 = {}", root2);
        } else if d == 0.0 {
            let root = -b / (2.0 * a);

            println!("The equation has one real root:");
            println!("Root = {}", root);
        } else {
            println!("The equation has no real roots.");
        }
    }
}