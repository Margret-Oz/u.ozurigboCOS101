// Project III: EFCC Dataset Merger
// Author: Ozurigbo Ugonna (Matric: 26120111047)

use std::fs::File;
use std::io::Write;

fn main() {
    // Separate datasets for commissioners, ministries, and zones
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let mut merged_output = String::new();
    let title = "EFCC - CONVICTED MINISTERS DATASET MERGE\n";
    let header = format!("{:<30} | {:<20} | {:<15}\n", "Minister Name", "Ministry", "Geopolitical Zone");
    let border = format!("{}\n", "-".repeat(72));

    merged_output.push_str(title);
    merged_output.push_str(&border);
    merged_output.push_str(&header);
    merged_output.push_str(&border);

    println!("{}", title);
    print!("{}", border);
    print!("{}", header);
    print!("{}", border);

    // Merge datasets by index loop
    for i in 0..commissioners.len() {
        let line = format!("{:<30} | {:<20} | {:<15}\n", commissioners[i], ministries[i], zones[i]);
        print!("{}", line);
        merged_output.push_str(&line);
    }

    print!("{}", border);
    merged_output.push_str(&border);

    // Write merged results into a file
    let mut file = File::create("efcc_merged_records.txt").expect("Failed to create merge file");
    file.write_all(merged_output.as_bytes()).expect("Failed to write merged data");

    println!("\nDatasets successfully merged and saved to 'efcc_merged_records.txt'.");
}