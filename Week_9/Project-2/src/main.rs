use std::fs::File;
use std::io::Write;

struct Student {
    name: &'static str,
    matric: &'static str,
    department: &'static str,
    level: u32,
}

fn main() {
    let students = vec![
        Student { name: "Oluchi Nnodi", matric: "ACC10211111", department: "Accounting", level: 300 },
        Student { name: "Adams Aliyu", matric: "ECO1010101", department: "Economics", level: 200 },
        Student { name: "Shania Bolade", matric: "CSC1032828", department: "Computer", level: 200 },
        Student { name: "Adenike Gold", matric: "EEE1020202", department: "Electrical", level: 300 },
        Student { name: "Blanca Edemoh", matric: "MEE1020201", department: "Mechanical", level: 100 },
    ];

    let mut file = File::create("students.txt").expect("Cannot create file");

    writeln!(file, "PAU SMIS STUDENT RECORDS\n").unwrap();

    for s in &students {
        writeln!(
            file,
            "{} ; {} ; {} ; Level {}",
            s.name, s.matric, s.department, s.level
        )
        .unwrap();
    }

    println!("File created: students.txt");
}
