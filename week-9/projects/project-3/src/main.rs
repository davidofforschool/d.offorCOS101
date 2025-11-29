use std::fs::File;
use std::io::Write;

// 1. Define a struct to represent a Student
struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

fn main() {
    // 2. Create a vector containing the student data from the image
    let students = vec![
        Student {
            name: String::from("Oluchi Mordi"),
            matric_number: String::from("ACC10211111"),
            department: String::from("Accounting"),
            level: 300,
        },
        Student {
            name: String::from("Adams Aliyu"),
            matric_number: String::from("ECO10110101"),
            department: String::from("Economics"),
            level: 100,
        },
        Student {
            name: String::from("Shania Bolade"),
            matric_number: String::from("CSC10328828"),
            department: String::from("Computer"),
            level: 200,
        },
        Student {
            name: String::from("Adekunle Gold"),
            matric_number: String::from("EEE11020202"),
            department: String::from("Electrical"),
            level: 200,
        },
        Student {
            name: String::from("Blanca Edemoh"),
            matric_number: String::from("MEE10202001"),
            department: String::from("Mechanical"),
            level: 100,
        },
    ];

    // 3. Display the details to the Console (Terminal)
    println!("PAU SMIS DATA DISPLAY");
    println!("============================================================");
    println!("{:<20} | {:<15} | {:<12} | {:<5}", "Student Name", "Matric. Number", "Department", "Level");
    println!("------------------------------------------------------------");
    
    for student in &students {
        println!("{:<20} | {:<15} | {:<12} | {:<5}", 
            student.name, 
            student.matric_number, 
            student.department, 
            student.level
        );
    }
    println!("============================================================");

    // 4. Save into a file (pau_smis.csv)
    let filename = "pau_smis.csv";
    
    // Create the file or panic with an error message if it fails
    let mut file = File::create(filename).expect("Could not create file");

    // Write the Main Header "PAU SMIS"
    // Note: We leave empty commas to merge columns visually in Excel, or just place it in the first cell.
    writeln!(file, "PAU SMIS").expect("Could not write header");
    
    // Write the Column Headers
    writeln!(file, "Student Name,Matric. Number,Department,Level").expect("Could not write columns");

    // Write the student data rows
    for student in &students {
        writeln!(file, "{},{},{},{}", 
            student.name, 
            student.matric_number, 
            student.department, 
            student.level
        ).expect("Could not write data row");
    }

    println!("\nData successfully saved to '{}'.", filename);
}