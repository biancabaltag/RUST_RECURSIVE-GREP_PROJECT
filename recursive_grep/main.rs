use regex::Regex;
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

fn procesare_input(
    path: &Path,
    substr_to_find: &str,
    count: bool,
    ignore_case: bool,
    regex_enabled: bool,
    max_lines: Option<usize>,
    lines_processed: &mut usize,
) -> io::Result<()> {
    if let Some(max) = max_lines {
        if *lines_processed >= max {
            return Ok(());
        }
    }

    if path.is_file() {
        println!("Cautam in fisierul: {}", path.display());
        let mut file = fs::File::open(&path)?;
        let mut continut_fisier = String::new();
        file.read_to_string(&mut continut_fisier)?;
        cautare(
            &continut_fisier,
            substr_to_find,
            count,
            ignore_case,
            regex_enabled,
            &path,
            max_lines,
            lines_processed,
        );
    } else if path.is_dir() {
        parcurgere_si_cautare(
            path,
            substr_to_find,
            count,
            ignore_case,
            regex_enabled,
            max_lines,
            lines_processed,
        )?;
    } else {
        eprintln!("Eroare: {} nu este un fisier sau director valid.", path.display());
    }
    Ok(())
}

fn parcurgere_si_cautare(
    dir: &Path,
    substr_to_find: &str,
    count: bool,
    ignore_case: bool,
    regex_enabled: bool,
    max_lines: Option<usize>,
    lines_processed: &mut usize,
) -> io::Result<()> {
    if let Ok(optiuni) = fs::read_dir(dir) {
        for optiune in optiuni {
            if let Some(max) = max_lines {
                if *lines_processed >= max {
                    println!("Limita de {} linii a fost atinsa. Oprire procesare.", max);
                    break;
                }
            }
            match optiune {
                Ok(entry) => {
                    let path = entry.path();
                    procesare_input(
                        &path,
                        substr_to_find,
                        count,
                        ignore_case,
                        regex_enabled,
                        max_lines,
                        lines_processed,
                    )?;
                }
                Err(e) => {
                    eprintln!("Eroare la citirea optiunii: {}", e);
                }
            }
        }
    } else {
        eprintln!("Eroare: Nu am putut citi directorul {}", dir.display());
    }
    Ok(())
}

fn cautare(
    continut_fisier: &str,
    substr_to_find: &str,
    count: bool,
    ignore_case: bool,
    regex_enabled: bool,
    path: &Path,
    max_lines: Option<usize>,
    lines_processed: &mut usize,
) {
    let mut match_count = 0;
    let regex = if regex_enabled {
        let pattern = if ignore_case {
            format!("(?i){}", substr_to_find)
        } else {
            substr_to_find.to_string()
        };
        match Regex::new(&pattern) {
            Ok(r) => Some(r),
            Err(e) => {
                eprintln!("Eroare in regex-ul introdus '{}': {}", substr_to_find, e);
                std::process::exit(1);
            }
        }
    } else {
        None
    };

    for (line_number, line) in continut_fisier.lines().enumerate() {
        let found = if let Some(ref regex) = regex {
            regex.is_match(line)
        } else if ignore_case {
            line.to_lowercase().contains(&substr_to_find.to_lowercase())
        } else {
            line.contains(substr_to_find)
        };

        if found {
            match_count += 1;

            if let Some(max) = max_lines {
                if *lines_processed >= max {
                    break;
                }
            }

            if !count {
                *lines_processed += 1;
                println!("linia {}: {}", line_number + 1, line);
            }
        }
    }

    if count && match_count > 0 {
        if let Some(max) = max_lines {
            let remaining = if *lines_processed > max {
                0
            } else {
                max - *lines_processed
            };
            let to_display = if match_count > remaining {
                remaining
            } else {
                match_count
            };

            if to_display > 0 {
                println!("{}: {}", path.display(), to_display);
                *lines_processed += to_display;
            }
        } else {
            println!("{}: {}", path.display(), match_count);
        }
    }
}

fn main() -> Result<(), io::Error> {
    let argumente: Vec<String> = env::args().collect();

    if argumente.len() < 3 || argumente.len() > 7 {
        eprintln!(
            "Utilizare: {} <cale_fisier_sau_dir> <substr_to_find> [--count] [--ignore-case] [--regex] [--max-lines=N]",
            argumente[0]
        );
        return Ok(());
    }

    let path = Path::new(&argumente[1]);
    let substr_to_find = &argumente[2];
    let count = argumente.contains(&String::from("--count"));
    let ignore_case = argumente.contains(&String::from("--ignore-case"));
    let regex_enabled = argumente.contains(&String::from("--regex"));
    let mut max_lines = None;

    for arg in &argumente {
        if arg.starts_with("--max-lines=") {
            if let Some(nr) = arg.split('=').nth(1) {
                max_lines = nr.parse::<usize>().ok();
            }
        }
    }

    if !path.exists() {
        eprintln!("Eroare: Calea {} nu exista.", argumente[1]);
        return Ok(());
    }

    let mut lines_processed = 0;
    procesare_input(
        path,
        substr_to_find,
        count,
        ignore_case,
        regex_enabled,
        max_lines,
        &mut lines_processed,
    )?;
    Ok(())
}
