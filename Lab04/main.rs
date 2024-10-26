use std::{fs, io};

fn longest_line() -> Result<(), io::Error> {
    let content = fs::read_to_string("input.txt")?;
    let mut longest_bytes = "";
    let mut longest_chars = "";

    for line in content.split('\n') {
        if line.as_bytes().len() > longest_bytes.as_bytes().len() {
            longest_bytes = line;
        }

        if line.chars().count() > longest_chars.chars().count() {
            longest_chars = line;
        }
    }

    println!(
        "Longest line by bytes: '{}', {} bytes",
        longest_bytes,
        longest_bytes.as_bytes().len()
    );
    println!(
        "Longest line by characters: '{}', {} characters",
        longest_chars,
        longest_chars.chars().count()
    );

    Ok(())
}

fn pb1() {
    match longest_line() {
        Ok(_) => println!(""),
        Err(e) => panic!("Failed: {}", e),
    }
}

fn pb2() -> Result<String, String> {
    let input_text = "Hello, World!\nUryyb, Jbeyq!";
    let mut output_text = String::new();

    for ch in input_text.chars() {
        if ch.is_ascii() {
            let rotated = if ch.is_lowercase() {
                let base = 'a' as u8;
                let offset = (ch as u8 - base + 13) % 26;
                (base + offset) as char
            } else if ch.is_uppercase() {
                let base = 'A' as u8;
                let offset = (ch as u8 - base + 13) % 26;
                (base + offset) as char
            } else {
                ch
            };
            output_text.push(rotated);
        } else {
            return Err(format!("error: invalid character: {}", ch));
        }
    }

    Ok(output_text)
}

fn pb3() -> Result<(), String> {
    let input_content = fs::read_to_string("input1.txt");
    let input_content = match input_content {
        Ok(content) => content,
        Err(e) => return Err(format!("error reading input1.txt: {}", e)),
    };

    let words: Vec<&str> = input_content.split_whitespace().collect();
    let mut output_content = String::new();

    for i in 0..words.len() {
        let word = words[i];
        let replacement = match word {
            "pt" => "pentru",
            "ptr" => "pentru",
            "dl" => "domnul",
            "dna" => "doamna",
            _ => word,
        };

        output_content.push_str(replacement);

        if i < words.len() - 1 {
            output_content.push(' ');
        }
    }

    println!("{}", output_content);

    match fs::write("output.txt", &output_content) {
        Ok(_) => println!("Transformation complete. Result written to output.txt"),
        Err(e) => return Err(format!("error writing to output.txt: {}", e)),
    }

    Ok(())
}

fn pb4() -> Result<(), String> {
    let file_path = "C:\\Windows\\System32\\drivers\\etc\\hosts";
    let file_content = fs::read_to_string(file_path);

    let file_content = match file_content {
        Ok(content) => content,
        Err(e) => return Err(format!("error reading file: {}", e)),
    };

    let lines: Vec<&str> = file_content.lines().collect();

    for line in lines {
        if line.is_empty() || (line.len() > 0 && &line[0..1] == "#") {
            continue;
        }

        let columns: Vec<&str> = line.split_whitespace().collect();
        if columns.len() >= 2 {
            let ip = columns[0];
            let hostname = columns[1];
            println!("{} => {}", hostname, ip);
        }
    }

    Ok(())
}

fn main() {
    println!("Problem 1-----------------------------------------------------");
    pb1();

    println!("Problem 2-----------------------------------------------------");
    match pb2() {
        Ok(rotated_text) => println!("ROT13 Result:\n{}", rotated_text),
        Err(e) => eprintln!("{}", e),
    }

    println!("Problem 3-----------------------------------------------------");
    match pb3() {
        Ok(_) => println!("Function executed successfully"),
        Err(e) => eprintln!("{}", e),
    }

    println!("Problem 4-----------------------------------------------------");
    match pb4() {
        Ok(_) => println!("Processing complete."),
        Err(e) => eprintln!("{}", e),
    }
}
