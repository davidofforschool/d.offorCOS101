use std::io;

fn main() {
    let aps_levels = vec![
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL1 8-10",
        "EL2 10-13",
        "SES",
    ];

    let professions = vec![
        "Office Administrator",
        "Academic",
        "Lawyer",
        "Teacher",
    ];

    let rank_table = vec![
        vec!["Intern", "-", "Paralegal", "Placement"],                  
        vec!["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"],
        vec!["Senior Administrator", "PhD Candidate", "Associate", "Snr Teacher"], 
        vec!["Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"], 
        vec!["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"], 
        vec!["CEO", "Dean", "Partner", "Principal"],                         
    ];

    let input_profession = get_input("Enter profession (e.g. Lawyer): ");
    let input_rank = get_input("Enter rank (e.g. Associate): ");
    let input_years = get_input("Enter years of experience: ");

    let years: u32 = input_years.trim().parse().unwrap_or(0);

    let prof_index = match professions.iter().position(|p| p.eq_ignore_ascii_case(input_profession.trim())) {
        Some(i) => i,
        None => {
            println!("Profession not found in table.");
            return;
        }
    };

    let mut found: Option<&str> = None;

    for (row, ranks) in rank_table.iter().enumerate() {
        if ranks[prof_index].eq_ignore_ascii_case(input_rank.trim()) {
            found = Some(aps_levels[row]);
            break;
        }
    }

    match found {
        Some(level) => println!(
            "\nResult:\nA {} with rank '{}' and {} years experience is classified as **{}**",
            input_profession.trim(),
            input_rank.trim(),
            years,
            level
        ),
        None => println!("No APS classification found for that rank + profession."),
    }
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);

    use std::io::Write;
    std::io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}