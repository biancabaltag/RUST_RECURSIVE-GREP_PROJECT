use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

fn parcurgere_si_cautare(
    dir: &Path,
    substr_to_find: &str,
    count: bool,
    ignore_case: bool,
    max_lines: Option<usize>,
    lines_processed: &mut usize,
) -> io::Result<()> {
    if let Ok(optiuni) = fs::read_dir(dir) {
        for optiune in optiuni {
            match optiune {
                Ok(entry) => {
                    let path = entry.path();
                    if path.is_dir() {
                        parcurgere_si_cautare(
                            &path,
                            substr_to_find,
                            count,
                            ignore_case,
                            max_lines,
                            lines_processed,
                        )?;
                    } else if path.is_file() {
                        if let Some(max) = max_lines {
                            if *lines_processed >= max {
                                break;
                            }
                        }
                        println!("CAUTAM IN FISIERUL: {}", path.display());
                        let mut file = fs::File::open(&path)?;
                        let mut continut_fisier = String::new();

                        file.read_to_string(&mut continut_fisier)?;
                        cautare(
                            &continut_fisier,
                            substr_to_find,
                            count,
                            ignore_case,
                            &path,
                            max_lines,
                            lines_processed,
                        );
                    }
                }
                Err(e) => {
                    eprintln!("Eroare la citirea opțiunii: {}", e);
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
    path: &Path,
    max_lines: Option<usize>,
    lines_processed: &mut usize,
) {
    let mut match_count = 0; // nr potriviri
    let mut linia_curenta = 1;
    let mut inceput_linie = 0;

    let sir_cautat_len = substr_to_find.len();
    let continut_len = continut_fisier.len();

    let sir_cautat_bytes = if ignore_case {
        substr_to_find.to_lowercase().as_bytes().to_vec()
    } else {
        substr_to_find.as_bytes().to_vec()
    };

    let continut_bytes = if ignore_case {
        continut_fisier.to_lowercase().into_bytes()
    } else {
        continut_fisier.as_bytes().to_vec()
    };

    let mut index = 0;

    while index < continut_len {
        let caracter = continut_bytes[index] as char;

        if caracter == '\n' {
            linia_curenta += 1;
            inceput_linie = index + 1;
        }

        if index + sir_cautat_len <= continut_len {
            if am_gasit(&continut_bytes, &sir_cautat_bytes, index) {
                match_count += 1;

                if let Some(max) = max_lines {
                    if *lines_processed >= max {
                        break; 
                    }
                }

                if !count {
                    *lines_processed += 1; 
                    let index_in_linie = index - inceput_linie;
                    let sfarsit_linie = continut_fisier[inceput_linie..]
                        .find('\n')
                        .unwrap_or(continut_len - inceput_linie);
                    let linie_text = &continut_fisier[inceput_linie..inceput_linie + sfarsit_linie];
                    println!(
                        "Am găsit substringul la linia {} și la indexul {}: {}",
                        linia_curenta, index_in_linie, linie_text
                    );
                }
            }
        }

        index += 1;
    }

    if count {
        if match_count > 0 {
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
}

fn am_gasit(continut_bytes: &[u8], sir_cautat_bytes: &[u8], start: usize) -> bool {
    for i in 0..sir_cautat_bytes.len() {
        if continut_bytes[start + i] != sir_cautat_bytes[i] {
            return false;
        }
    }
    true
}

fn main() -> Result<(), io::Error> {
    let argumente: Vec<String> = env::args().collect();

    if argumente.len() < 3 || argumente.len() > 6 {
        eprintln!(
            "Utilizare: {} <cale_dir> <substr_to_find> [--count] [--ignore-case] [--max-lines=N]",
            argumente[0]
        );
        return Ok(());
    }

    let dir = Path::new(&argumente[1]);
    let substr_to_find = &argumente[2];
    let count = argumente.contains(&String::from("--count"));
    let ignore_case = argumente.contains(&String::from("--ignore-case"));
    let mut max_lines = None;

    for arg in &argumente {
        if arg.starts_with("--max-lines=") {
            if let Some(nr) = arg.split('=').nth(1) {
                max_lines = nr.parse::<usize>().ok();
            }
        }
    }

    if !dir.exists() {
        eprintln!("Eroare: Directorul {} nu există.", argumente[1]);
        return Ok(());
    }

    if !dir.is_dir() {
        eprintln!("Eroare: {} nu este un director valid.", argumente[1]);
        return Ok(());
    }

    let mut lines_processed = 0; // contor global pentru linii procesate
    parcurgere_si_cautare(
        dir,
        substr_to_find,
        count,
        ignore_case,
        max_lines,
        &mut lines_processed,
    )?;
    Ok(())
}
