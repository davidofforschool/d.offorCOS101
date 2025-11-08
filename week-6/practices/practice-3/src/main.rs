fn main() {
    let name1 = "David Offor"
    println!("My name is {}", name1);

    // find and replace
    let name2 = name1.replace("Ayomide","Adebare");
    println!("you can also call me {}", name2);
    let faculty = "faculty of Science and technology";

    let school = faculty.replace("faculty","School");
    println!("I am a student of {}",school);
}