// ==========================================
// Project I: The APS Level Checker
// CSC 101 Practical - Pan-Atlantic University
// ==========================================
// This program is built for a team working with the Federal Government 
// of Nigeria. It uses Rust vectors to map out and validate public service 
// staff grades based on their profession, job title, and years of experience.

fn main() {
    // Step 1: Define the overarching career level tiers (our rows/levels)
    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
    
    // Step 2: Define the major profession tracks available in the public service (our columns)
    let professions = vec!["Office Admin", "Academic", "Lawyer", "Teacher"];

    // Step 3: Map out the job titles for each profession across the 6 progression tiers.
    // Each vector corresponds to a profession track from entry level up to executive management.
    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["Paralegal", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Placement", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Classroom Teacher", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    // Step 4: Bundle all profession vectors into a single organizational vector of vectors.
    // This allows us to look up specific career paths dynamically using indexes.
    let organization = vec![office_admin, academic, lawyer, teacher];

    // --- STAFF QUERY SIMULATION ---
    // Let's test a real scenario: Checking an Associate Lawyer with 6 years of experience.
    let prof_index = 2;              // Index 2 maps to "Lawyer" in our professions vector
    let target_title = "Associate";    // The title we want to verify
    let years_exp = 6;               // The staff member's years of service

    // Print out a neat header for our checker tool
    println!("==========================================");
    println!(" FEDERAL GOVERNMENT OF NIGERIA APS CHECKER");
    println!("==========================================");
    println!("-> Profession Track: {}", professions[prof_index]);
    println!("-> Submitted Title:  {}", target_title);
    println!("-> Years on Job:     {} years\n", years_exp);

    // Step 5: Borrow the specific profession's vector using its index
    let staff_roles = &organization[prof_index];
    
    // Step 6: Validate the staff member's credentials. 
    // According to the table, index 2 ("APS 5-8") corresponds to 5 to 8 years of experience.
    // We check if the years fall in range AND if the title matches the slot at index 2.
    if years_exp >= 5 && years_exp <= 8 && staff_roles[2] == target_title {
        println!(">>> Validation Result: SUCCESS!");
        println!(">>> The staff member officially holds position: {} ({})[cite: 1]", levels[2], target_title);
    } else {
        println!(">>> Validation Result: FAILED!");
        println!(">>> Mismatch detected between years of experience and the claimed title.");
    }
}