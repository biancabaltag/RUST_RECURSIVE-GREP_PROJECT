use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

fn parcurgere_si_cautare(dir: &Path, substr_to_find: &str, count: bool) -> io::Result<()> {

    if let Ok(optiuni) = fs::read_dir(dir) {

        for optiune in optiuni {

            match optiune {

                Ok(entry) => {

                    let path = entry.path();
                    if path.is_dir() {

                        parcurgere_si_cautare(&path, substr_to_find, count)?;
                    } else if path.is_file() {

                        println!("CAUTAM IN FISIERUL: {}", path.display());

                        let mut file = fs::File::open(&path)?;
                        let mut continut_fisier = String::new();
                        
                        file.read_to_string(&mut continut_fisier)?;
                        cautare(&continut_fisier, substr_to_find, count, &path);
                    }
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

fn cautare(continut_fisier: &str, substr_to_find: &str, count: bool, path: &Path) {

    let mut match_count = 0; // nr potriviri
    let mut linia_curenta = 1;
    let mut inceput_linie = 0;

    let sir_cautat_len = substr_to_find.len();
    let continut_len = continut_fisier.len();
    
    let sir_cautat_bytes = substr_to_find.as_bytes();
    let continut_bytes = continut_fisier.as_bytes();

    let mut index = 0;

    while index < continut_len {
        let caracter = continut_bytes[index] as char;

        if caracter == '\n' {
            linia_curenta += 1;
            inceput_linie = index + 1;
        }

        if index + sir_cautat_len <= continut_len {
            if am_gasit(continut_bytes, sir_cautat_bytes, index) {

                match_count += 1; // incrementam contorul
                if !count { // daca nu este modul count, afisam liniile

                    let index_in_linie = index - inceput_linie;
                    let sfarsit_linie = continut_fisier[inceput_linie..].find('\n').unwrap_or(continut_len - inceput_linie);
                    let linie_text = &continut_fisier[inceput_linie..inceput_linie + sfarsit_linie];
                    println!(
                        "Am gasit substringul la linia {} si la indexul {}: {}",
                        linia_curenta, index_in_linie, linie_text
                    );
                }
            }
        }

        index += 1;
    }

    if count {
        // daca este modul count, afisam doar numarul total de potriviri
        println!("{}: {}", path.display(), match_count);
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

    if argumente.len() < 3 || argumente.len() > 4 {
        eprintln!("Utilizare: {} <cale_dir> <substr_to_find> [--count]", argumente[0]);
        eprintln!("Te rugam să verifici formatul si sa incerci din nou.");
        return Ok(()); // continuam executia
    }


    let dir = Path::new(&argumente[1]);
    let substr_to_find = &argumente[2];
    let count = argumente.get(3).map_or(false, |arg| arg == "--count");

    if !dir.exists() {
        eprintln!(
            "Eroare: nu exista acest director: {}. Te rugam sa verifici.",
            argumente[1]
        );
        eprintln!("Utilizare: {} <cale_dir> <substr_to_find> [--count]", argumente[0]);
        return Ok(()); // continuam executia
    }

    if !dir.is_dir() {
        eprintln!(
            "Eroare: nu este director: {}. Te rugăm să verifici.",
            argumente[1]
        );
        eprintln!("Utilizare: {} <cale_dir> <substr_to_find> [--count]", argumente[0]);
        return Ok(()); // continuam executia
    }

    parcurgere_si_cautare(dir, substr_to_find, count)?;
    Ok(())
}
