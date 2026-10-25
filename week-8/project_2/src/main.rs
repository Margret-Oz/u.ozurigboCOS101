// Project II: EY Nigeria Developer Hiring Evaluator
// Author: Ozurigbo Ugonna (Matric: 26120111047)

fn main() {
    println!("=== EY Nigeria Developer Interview Evaluation ===");

    // Using a vector of tuples: (Candidate Name, Years of Experience)
    let candidates: Vec<(&str, u32)> = vec![
        ("Chidera Obi", 4),
        ("Amina Bello", 7),
        ("Tunde Bakare", 5),
        ("Ejiro Oghene", 9),
        ("Fatima Yusuf", 6),
    ];

    if candidates.is_empty() {
        println!("No candidates found in the pool.");
        return;
    }

    // Initialize tracking variables with the first candidate
    let mut top_candidate = candidates[0].0;
    let mut max_experience = candidates[0].1;

    // Walk through the vector using a for loop
    for i in 1..candidates.len() {
        let current_name = candidates[i].0;
        let current_exp = candidates[i].1;

        if current_exp > max_experience {
            max_experience = current_exp;
            top_candidate = current_name;
        }
    }

    println!("\nInterview Results Summary:");
    for (name, exp) in &candidates {
        println!(" - {} with {} years of experience", name, exp);
    }

    println!("\n[Selection]: The candidate with the highest years of programming experience is **{}** with **{} years** of experience.", top_candidate, max_experience);
}