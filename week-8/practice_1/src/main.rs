fn main() {
    // Using Vec::new()
    let _v : Vec<i64> = Vec::new();

    // printing the size of vector
    println!("\nThe length of Vec::new");

    // Using macro
    let v = vec!["Grace","Effiong","Basil","Kareem","Susan"];

    //printing the size of vector
    println!("\nThe length of vec macro is: {}",v.len());
}