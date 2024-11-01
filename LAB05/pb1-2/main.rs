use std::{fs, io};
use serde::{Deserialize, Serialize};
use serde_json;





#[derive(Debug, Clone)]
struct Student<'a> {
    name: &'a str,
    phone_number: &'a str, 
    age: i32,
}

const INALTIME_CANVAS: usize = 55;
const LATIME_CANVAS: usize = 100;

struct Canvas {
    grila: [[char; LATIME_CANVAS]; INALTIME_CANVAS],
}

fn nou() -> Canvas {
    Canvas {
        grila: [[' '; LATIME_CANVAS]; INALTIME_CANVAS],
    }
}

fn seteaza_pixel(canvas: &mut Canvas, x: usize, y: usize, valoare: char) {
    if x < LATIME_CANVAS && y < INALTIME_CANVAS {
        canvas.grila[y][x] = valoare;
    } else {
        println!("Eroare: Coordonatele ({}, {}) sunt in afara limitelor", x, y);
    }
}

fn afiseaza(canvas: &Canvas) {
    for rand in &canvas.grila {
        let mut has_content = false;

        for &pixel in rand {
            if pixel != ' ' {
                has_content = true;
                break;
            }
        }

        if has_content {
            for &pixel in rand {
                print!("{}", pixel);
            }
            println!();
        }
    }
}

fn pb2() {
    let mut canvas = nou();
    seteaza_pixel(&mut canvas, 0, 0, 'X');
    seteaza_pixel(&mut canvas, 1, 0, 'O');
    seteaza_pixel(&mut canvas, 2, 0, '#');
    seteaza_pixel(&mut canvas, 3, 0, '*');
    seteaza_pixel(&mut canvas, 4, 0, '@');
    seteaza_pixel(&mut canvas, 10, 0, 'X');
    seteaza_pixel(&mut canvas, 31, 0, 'O');
    seteaza_pixel(&mut canvas, 25, 3, '#');
    seteaza_pixel(&mut canvas, 23, 0, '*');
    seteaza_pixel(&mut canvas, 12, 0, '@');
    seteaza_pixel(&mut canvas, 10, 45, 'X');
    seteaza_pixel(&mut canvas, 31, 22, 'O');
    seteaza_pixel(&mut canvas, 25, 41, '#');
    seteaza_pixel(&mut canvas, 23, 43, '*');
    seteaza_pixel(&mut canvas, 12, 42, '@');

    println!("Iesire Canvas:");
    afiseaza(&canvas);
}






fn manage_lines(line: &str) -> Student {
    
    let mut stud_info: [&str; 3] = ["", "", ""]; 

    
    let mut count = 0;
    for info in line.split(',') {
        if count < 3 {
            stud_info[count] = info; 
            count += 1;
        }
    }

    
    let name = stud_info[0];
    let phone_number = stud_info[1]; 
    let age = match stud_info[2].parse::<i32>() {
        Ok(age) => age,   
        Err(_) => 0,     
    };

    Student { name, phone_number, age }
}

fn pb1() -> Result<(), io::Error> {
    
    let content = fs::read_to_string("studenti.txt")?;

    
    let mut youngest_student = Student { name: "", phone_number: "", age: i32::MAX }; 
    let mut oldest_student = Student { name: "", phone_number: "", age: i32::MIN };

   
    for line in content.lines() {
        let student = manage_lines(line);

        
        if student.age < youngest_student.age {
            youngest_student = student.clone(); 
        }

        
        if student.age > oldest_student.age {
            oldest_student = student.clone(); 
        }
    }

    if youngest_student.age > 0 {
        println!("The youngest student is: {}, {}", youngest_student.name, youngest_student.phone_number);
    } else {
        println!("No valid students.");
    }

    if oldest_student.age > 0 {
        println!("The oldest student is: {}, {}", oldest_student.name, oldest_student.phone_number);
    } else {
        println!("No valid students.");
    }

    Ok(())
}




fn main() {
    println!("PROBLEMA1 -----------------------");
    match pb1() {
        Ok(_) => println!("Finished..."),
        Err(e) => println!("Failed... {}", e),
    }

    println!("PROBLEMA2 -----------------------");
    pb2();




    

    
}
