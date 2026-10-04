use std::io;

fn main() {
    println!("===== RESTAURANT MENU =====");
    println!("P - Poundo Yam / Edinkaiko Soup - N3200");
    println!("F - Fried Rice & Chicken       - N3000");
    println!("A - Amala & Ewedu Soup         - N2500");
    println!("E - Eba & Egusi Soup           - N2000");
    println!("W - White Rice & Stew           - N2500");

    let mut food_type = String::new();
    let mut quantity_input = String::new();

    println!("\nEnter the food type (P, F, A, E or W):");
    io::stdin()
        .read_line(&mut food_type)
        .expect("Failed to read input");

    println!("Enter quantity:");
    io::stdin()
        .read_line(&mut quantity_input)
        .expect("Failed to read input");

    let food_type = food_type.trim().to_uppercase();
    let quantity: i32 = quantity_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let price: i32;

    if food_type == "P" {
        price = 3200;
    } else if food_type == "F" {
        price = 3000;
    } else if food_type == "A" {
        price = 2500;
    } else if food_type == "E" {
        price = 2000;
    } else if food_type == "W" {
        price = 2500;
    } else {
        println!("Invalid food type.");
        return;
    }

    let total = price * quantity;

    println!("\nQuantity: {}", quantity);
    println!("Price per item: N{}", price);
    println!("Total before discount: N{}", total);

    if total > 10000 {
        let discount = total * 5 / 100;
        let final_total = total - discount;

        println!("Discount (5%): N{}", discount);
        println!("Total to pay: N{}", final_total);
    } else {
        println!("No discount applied.");
        println!("Total to pay: N{}", total);
    }
}