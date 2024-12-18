use regex::Regex;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

struct OptionCommand<'a> {
    //struct pt a retine optiunile selectate de utilzator
    substr_to_find: &'a str,
    count: bool,
    ignore_case: bool,
    regex_op: bool,
    max_lines: Option<usize>,
}

fn help() {
    // functia helo
    println!("Utilizare: recursive_grep <cale_fisier_sau_dir> <substr_to_find> [optiuni]");
    println!();
    println!("Optiuni disponibile:");
    println!("  --count          nr de valori pentru fiecare fisier");
    println!("  --ignore-case    not sensetive case");
    println!("  --regex          interpretare expresie regulata");
    println!(
        "  --max-lines=N    se opreste din cautarea in fiser dupa un anumit nr de linii gasite"
    );
    println!();
}

fn procesare_input(
    //in functie de directorul/fisierul primit ca input (functia este reapelata atunci cand vrem sa cautam in fisier)
    path: &Path,
    context: &OptionCommand,
    total_matches: &mut usize,
) -> Result<(), String> {
    if path.is_file() {
        // daca e fiser il deschidem, si aplicam functia de cautare asupra sa
        let mut file = File::open(path)
            .map_err(|_| format!("eroare la deschiderea fisierului: {}", path.display()))?;
        let mut continut_fisier = String::new();

        file.read_to_string(&mut continut_fisier).map_err(|_| {
            format!(
                "fisierul {} nu este UTF-8 sau nu poate fi citit.",
                path.display()
            )
        })?;
        cautare(&continut_fisier, path, context, total_matches);
    } else if path.is_dir() {
        // daca e director apelam o functia care va cauta recursiv
        parcurgere_si_cautare(path, context, total_matches)?;
    } else {
        return Err(format!(
            "{} nu este un fisier sau director valid.",
            path.display()
        ));
    }
    Ok(())
}

fn parcurgere_si_cautare(
    dir: &Path,
    context: &OptionCommand,
    total_matches: &mut usize,
) -> Result<(), String> {
    // functia imparte un director in doi vectori, de subdirectoare si fisiere
    let mut subdirectoare = Vec::new();
    let mut fisiere = Vec::new();

    for optiune in fs::read_dir(dir)
        .map_err(|_| format!("eroare la citirea directorului : {}", dir.display()))?
    {
        let entry = optiune.map_err(|_| {
            format!(
                "eroare la procesarea unei intrari din directorul: {}",
                dir.display()
            )
        })?;
        let path = entry.path();

        if path.is_dir() {
            subdirectoare.push(path);
        } else if path.is_file() {
            fisiere.push(path);
        }
    }

    for subdir in subdirectoare {
        //parcurge intai directoarele pentru a ajunge la cel mai indepartat fisier
        parcurgere_si_cautare(&subdir, context, total_matches)?; //apelare recursiva
    }

    for fisier in fisiere {
        procesare_input(&fisier, context, total_matches)?;
    }

    Ok(())
}

fn cautare(continut_fisier: &str, path: &Path, context: &OptionCommand, total_matches: &mut usize) {
    let mut match_count = 0;
    let mut lines_in_file = 0;

    let regex = if context.regex_op {
        //daca e regex, aplica anumite schimbari asupra lui
        let pattern = if context.ignore_case {
            format!("(?i){}", context.substr_to_find)
        } else {
            context.substr_to_find.to_string()
        };
        match Regex::new(&pattern) {
            Ok(r) => Some(r),
            Err(e) => {
                eprintln!(
                    "eroare in regex-ul introdus '{}': {}",
                    context.substr_to_find, e
                );
                std::process::exit(1);
            }
        }
    } else {
        None //daca e string, nu face nicio schimbare
    };

    for (line_number, line) in continut_fisier.lines().enumerate() {
        let found = if let Some(ref regex) = regex {
            regex.is_match(line)
        } else if context.ignore_case {
            line.to_lowercase()
                .contains(&context.substr_to_find.to_lowercase())
        } else {
            line.contains(context.substr_to_find)
        };

        if found {
            // daca au match, atunci incrementam nr de match uri din fisier
            lines_in_file += 1;

            if context.max_lines.is_some() && lines_in_file > context.max_lines.unwrap() {
                break; // daca am ajuns la nr maxim de linii iesim
            }

            match_count += 1;

            if !context.count {
                println!("{}: Linia {}: {}", path.display(), line_number + 1, line);
            }
        }
    }

    if context.count {
        // o optiune bonus, in care sunt afisate si match urile totale din toate fisierele
        println!("{}: {}", path.display(), match_count);
        *total_matches += match_count;
    }
}

fn main() -> Result<(), String> {
    let argumente: Vec<String> = env::args().collect();

    if argumente.contains(&String::from("--help")) {
        help();
        return Ok(());
    }

    if argumente.len() < 3 || argumente.len() > 7 {
        return Err("eroare: Numar invalid de argumente.".to_string());
    }

    //sunt extrase toate arg din linia de comanda si stocate intr un struct definit anterior

    let path = Path::new(&argumente[1]);
    let substr_to_find = &argumente[2];
    let count = argumente.contains(&String::from("--count"));
    let ignore_case = argumente.contains(&String::from("--ignore-case"));
    let regex_op = argumente.contains(&String::from("--regex"));
    let mut max_lines = None;

    for arg in &argumente {
        if arg.starts_with("--max-lines=") {
            //validarea constantei de dupa =
            let value = arg.split('=').nth(1);
            match value {
                Some(num) if num.chars().all(char::is_numeric) => {
                    max_lines = num.parse::<usize>().ok();
                }
                _ => {
                    return Err("--max-lines trebuie sa fie un numar valid.".to_string());
                }
            }
        }
    }

    if !path.exists() {
        //verificare existenta cale
        return Err(format!("calea {} nu exista.", argumente[1]));
    }

    //stocarea informatiilor despre comanda utilizatorului

    let context = OptionCommand {
        substr_to_find,
        count,
        ignore_case,
        regex_op,
        max_lines,
    };

    let mut total_matches = 0;
    procesare_input(path, &context, &mut total_matches)?;

    //afisare total potriviri daca optiunea count este true
    if context.count {
        println!("total potriviri: {}", total_matches);
    }

    Ok(())
}
