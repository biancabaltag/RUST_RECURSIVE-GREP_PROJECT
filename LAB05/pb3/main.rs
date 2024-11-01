use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Clone)]
struct Student {
    name: String,
    phone: String,
    age: i32,
}

fn parse_student(line: &str) -> Option<Student> {
    match serde_json::from_str::<Student>(line) {
        Ok(student) => Some(student),
        Err(_) => None,
    }
}

fn find_oldest_and_youngest(filename: &str) -> Result<(Option<Student>, Option<Student>), io::Error> {
    let content = fs::read_to_string(filename)?;
    let mut youngest_student: Option<Student> = None;
    let mut oldest_student: Option<Student> = None;

    for line in content.lines() {
        if let Some(student) = parse_student(line) {
            youngest_student = match youngest_student {
                Some(ref y) if y.age <= student.age => Some(y.clone()),
                _ => Some(student.clone()),
            };
            oldest_student = match oldest_student {
                Some(ref o) if o.age >= student.age => Some(o.clone()),
                _ => Some(student.clone()),
            };
        }
    }

    Ok((youngest_student, oldest_student))
}

fn main() -> Result<(), io::Error> {
    let (youngest_student, oldest_student) = find_oldest_and_youngest("studenti.json")?;

    if let Some(youngest) = youngest_student {
        println!("The youngest student is: {}, {}, {}", youngest.name, youngest.phone, youngest.age);
    } else {
        println!("No valid students found.");
    }

    if let Some(oldest) = oldest_student {
        println!("The oldest student is: {}, {}, {}", oldest.name, oldest.phone, oldest.age);
    } else {
        println!("No valid students found.");
    }

    Ok(())
}
