fn adauga_spatiu(s: &mut String, n: usize) {
    let mut i = 0;
    while i < n {
        s.push(' ');
        i += 1;
    }
}

fn adauga_str(s: &mut String, adauga: &str) {
    let b = adauga.as_bytes();
    let mut i = 0;
    while i < b.len() {
        s.push(b[i] as char);
        i += 1;
    }
}

fn adauga_intreg(s: &mut String, mut numar: u32) {
    if numar == 0 {
        s.push('0');
        return;
    }

    let mut cifre: Vec<char> = Vec::new();
    let mut contor = 0;

    while numar > 0 {
        if contor == 3 {
            cifre.push('_');  
            contor = 0;
        }
        let cifra_caracter = (numar % 10) as u8 + b'0';  
        cifre.push(cifra_caracter as char);  
        numar /= 10;
        contor += 1;
    }

    let mut i = cifre.len();
    while i > 0 {
        i -= 1;
        s.push(cifre[i]);
    }
}

fn adauga_float(s: &mut String, numar: f32) {
    let parte_intreaga = numar as u32;
    let parte_zecimala = ((numar - parte_intreaga as f32) * 1000.0).round() as u32;

    adauga_intreg(s, parte_intreaga);
    s.push('.');

    let mut divizor = 100;
    while divizor > 1 && parte_zecimala < divizor {
        s.push('0');
        divizor /= 10;
    }

    adauga_intreg(s, parte_zecimala);
}

fn main() {
    let mut rezultat = String::new();

    adauga_spatiu(&mut rezultat, 40);
    adauga_str(&mut rezultat, "I 💚\n");
    
    adauga_spatiu(&mut rezultat, 40);
    adauga_str(&mut rezultat, "RUST.\n\n");

    adauga_spatiu(&mut rezultat, 4);
    adauga_str(&mut rezultat, "Most");
    adauga_spatiu(&mut rezultat, 12);
    adauga_str(&mut rezultat, "crate");
    adauga_spatiu(&mut rezultat, 6);
    adauga_intreg(&mut rezultat, 306437968);
    adauga_spatiu(&mut rezultat, 11);
    adauga_str(&mut rezultat, "and");
    adauga_spatiu(&mut rezultat, 5);
    adauga_str(&mut rezultat, "lastest");
    adauga_spatiu(&mut rezultat, 9);
    adauga_str(&mut rezultat, "is\n");

    adauga_spatiu(&mut rezultat, 9);
    adauga_str(&mut rezultat, "downloaded");
    adauga_spatiu(&mut rezultat, 8);
    adauga_str(&mut rezultat, "has");
    adauga_spatiu(&mut rezultat, 13);
    adauga_str(&mut rezultat, "downloads");
    adauga_spatiu(&mut rezultat, 6);
    adauga_str(&mut rezultat, "the");
    adauga_spatiu(&mut rezultat, 9);
    adauga_str(&mut rezultat, "version");
    adauga_spatiu(&mut rezultat, 4);
    adauga_float(&mut rezultat, 2.038);
    adauga_str(&mut rezultat, ".\n");

    println!("{}", rezultat);
}
