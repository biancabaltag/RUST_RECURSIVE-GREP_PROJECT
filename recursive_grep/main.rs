use std::fs;
use std::io::{self, Read};
use std::path::Path;

fn parcurgere_si_cautare(director: &Path, sir_cautat: &str) -> io::Result<()> {
    if let Ok(optiuni) = fs::read_dir(director) {

        for optiune in optiuni {

            match optiune {

                Ok(entry) => {

                    let path = entry.path();

                    if path.is_dir() {

                        // daca este director apelam recursiv functia
                        parcurgere_si_cautare(&path, sir_cautat)?;

                    } else if path.is_file() {

                        // daca este fisier, citim continutul si cautam
                        println!("CAUTAM IN FISIERUL: {}", path.display());
                        let mut file = fs::File::open(&path)?;
                        let mut continut_fisier = String::new();

                        file.read_to_string(&mut continut_fisier)?;
                        cautare(&continut_fisier, sir_cautat);
                    }
                }
                Err(e) => {
                    eprintln!("Eroare la citirea/directorului fisierului de optiune: {}", e);
                }
            }
        }
    }
    Ok(())
}

fn cautare(continut_fisier: &str, sir_cautat: &str) {
  
    let mut linia_curenta = 1;
    let mut inceput_linie = 0;
    let sir_cautat_len = sir_cautat.len();
    let continut_len = continut_fisier.len();
    let sir_cautat_bytes = sir_cautat.as_bytes();
    let continut_bytes = continut_fisier.as_bytes();

    let mut index = 0;

    while index < continut_len {
        let caracter = continut_bytes[index] as char;

        // trecem la urmatoarea linie
        if caracter == '\n' {
            linia_curenta += 1;
            inceput_linie = index + 1;
        }

        // verificam prin alg naiv daca exista substring-ul la pozitia respectiva
        if index + sir_cautat_len <= continut_len {
            if am_gasit(continut_bytes, sir_cautat_bytes, index) {
                // daca am gasit o portivere, afisam 
                let index_in_linie = index - inceput_linie;
                let sfarsit_linie = continut_fisier[inceput_linie..]
                    .find('\n')
                    .unwrap_or(continut_len - inceput_linie);
                let linie_text = &continut_fisier[inceput_linie..inceput_linie + sfarsit_linie];

                println!(
                    "Am gasit substringul la linia {} si la indexul {}: {}",
                    linia_curenta, index_in_linie, linie_text
                );
            }
        }

        index += 1;
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
    let director = Path::new("text");
    let sir_cautat = "pe";

    parcurgere_si_cautare(director, sir_cautat)?;
    Ok(())
}
