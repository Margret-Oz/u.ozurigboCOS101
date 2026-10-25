// Project II: PAU-SMIS Record Manager
// Author: Ozurigbo Ugonna (Matric: 26120111047)

use std::fs::File;
use std::io::Write;

fn main() {
    // Student data stored in a vector of tuples: (Name, Matric Number, Department, Level)
    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10328828", "Computer", 200),
        ("Adekunle Gold", "EEE11020202", "Electrical", 200),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", 100),
    ];

    let mut report_content = String::new();
    
    // Header formatting
    let header = format!("{:<20} | {:<15} | {:<12} | {:<5}\n", "Student Name", "Matric. Number", "Department", "Level");
    let separator = format!("{}\n", "-".repeat(60));

    println!("Displaying Student Records on Screen:");
    print!("{}", separator);
    print!("{}", header);
    print!("{}", separator);

    report_content.push_str(&separator);
    report_content.push_str(&header);
    report_content.push_str(&separator);

    // Read from vector and display
    for student in &students {
        let row = format!("{:<20} | {:<15} | {:<12} | {:<5}\n", student.0, student.1, student.2, student.3);
        print!("{}", row);
        report_content.push_str(&row);
    }
    print!("{}", separator);
    report_content.push_str(&separator);

    // Write data to file
    let mut file = File::create("pau_smis_records.txt").expect("Failed to create SMIS file");
    file.write_all(report_content.as_bytes()).expect("Failed to write SMIS data");

    println!("\nStudent records successfully saved to 'pau_smis_records.txt'.");
}