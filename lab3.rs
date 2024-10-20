fn is_prime(n: u16) -> bool {
    let mut i: u16 = 3;
    if n == 2 {
        return true;
    }
    if n == 0 || n == 1 || n % 2 == 0 {
        return false;
    }

    while (i as u32) * (i as u32) <= (n as u32) {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    return true;
}

fn urm_prime(x: u16) -> Option<u16> {
    let mut n = x + 1;

    if n == u16::MAX && is_prime(n) {
        return Some(n);
    }

    while n < u16::MAX {
        if is_prime(n) {
            return Some(n);
        }
        n += 1;
    }
    return None;
}

fn problema1() {
    let mut x: u16 = 60000;

    /*pentru a testa automat*/
    while let Some(prime) = urm_prime(x) {
        println!("Urmatorul nr prim este: {prime}");
        x = prime;
    }

    /*  match pentru cele doua cazuri care testeaza si afiseaza un raspuns ( match-ulpoate fi folosit individual, fara testarea automata, daca vrem sa te testam pentru un singur numar) */
    match urm_prime(x) {
        Some(prime) => println!("urmatorul nr prim dupa {x} este {prime}"),
        None => println!("eroare, urm nr prim nu este in u16"),
    }
}

fn check_add(a: u32, b: u32) -> u32 {
    if a > u32::MAX - b {
        panic!("Overflow for add");
    }
    {
        return a + b;
    }
}

fn check_mul(a: u32, b: u32) -> u32 {
    if a != 0 && b > u32::MAX / a {
        panic!("Overflow for mul");
    } else {
        return a * b;
    }
}

fn problema2() {
    let add1 = check_add(100, 2);
    println!("Sum: {}", add1);

    let mul1 = check_mul(100, 2);
    println!("MUL: {}", mul1);

    //let _add2 = check_add(u32::MAX, 2);  va da o eroare din cauza panic ului si va opri executia intregului program

    //let _mul2 = check_mul(u32::MAX, 2);
}

#[derive(Debug)]
enum MyError {
    OverFlow,
}

fn check_add1(a: u32, b: u32) -> Result<u32, MyError> {
    if a > u32::MAX - b {
        Err(MyError::OverFlow)
    } else {
        Ok(a + b)
    }
}

fn check_mul1(a: u32, b: u32) -> Result<u32, MyError> {
    if a != 0 && b > u32::MAX / a {
        Err(MyError::OverFlow)
    } else {
        Ok(a * b)
    }
}

fn problema3() {
    let a: u32 = 100;
    let b: u32 = 100;

    match check_add1(a, b) {
        Ok(sum) => println!("Sum: {}", sum),
        Err(e) => eprintln!("Error: {:?}", e),
    }

    match check_mul1(a, b) {
        Ok(mul) => println!("Mul: {mul}"),
        Err(e) => eprintln!("Err: {:?}", e),
    }

    match check_add1(u32::MAX, b) {
        Ok(sum) => println!("Sum: {}", sum),
        Err(e) => eprintln!("Error: {:?}", e),
    }
    match check_mul1(u32::MAX, b) {
        Ok(mul) => println!("Mul: {mul}"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

#[derive(Debug)]
enum MyError2 {
    NotAscii,
    NotDigit,
    NotBase16Digit,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<char, MyError2> {
    if c.is_ascii_alphabetic() {
        Ok(c.to_ascii_uppercase())
    } else {
        Err(MyError2::NotLetter)
    }
}

fn to_lowercase(c: char) -> Result<char, MyError2> {
    if c.is_ascii_alphabetic() {
        Ok(c.to_ascii_lowercase())
    } else {
        Err(MyError2::NotLetter)
    }
}

fn print_char(c: char) -> Result<(), MyError2> {
    if c.is_ascii_graphic() {
        print!("{}", c);
        Ok(())
    } else {
        Err(MyError2::NotPrintable)
    }
}

fn char_to_number(c: char) -> Result<u32, MyError2> {
    if c.is_ascii() {
        if c.is_digit(10) {
            Ok(c.to_digit(10).unwrap())
        } else {
            Err(MyError2::NotDigit)
        }
    } else {
        Err(MyError2::NotAscii)
    }
}

fn char_to_number_hex(c: char) -> Result<u32, MyError2> {
    if c.is_ascii() {
        if c.is_digit(16) {
            Ok(c.to_digit(16).unwrap())
        } else {
            Err(MyError2::NotBase16Digit)
        }
    } else {
        Err(MyError2::NotAscii)
    }
}

fn print_error(err: MyError2) {
    match err {
        MyError2::NotAscii => println!("Error: caracterul nu este ASCII..."),
        MyError2::NotDigit => println!("Error: caracterul nu este un digit..."),
        MyError2::NotBase16Digit => println!("Error: caracterul nu este un digit în baza 16..."),
        MyError2::NotLetter => println!("Error: caracterul nu este o litera..."),
        MyError2::NotPrintable => println!("Error: caracterul nu este imprimabil..."),
    }
}

fn problema4() {
    match to_uppercase('a') {
        Ok(ch) => println!("Uppercase a: {}", ch),
        Err(e) => print_error(e),
    }
    match to_uppercase('9') {
        Ok(ch) => println!("Uppercase 9: {}", ch),
        Err(e) => print_error(e),
    }

    match to_lowercase('R') {
        Ok(ch) => println!("Lowercase R: {}", ch),
        Err(e) => print_error(e),
    }
    match to_lowercase('?') {
        Ok(ch) => println!("Lowercase ?: {}", ch),
        Err(e) => print_error(e),
    }

    match print_char('A') {
        Ok(()) => println!(" A- Character printed"),
        Err(e) => print_error(e),
    }

    match char_to_number('6') {
        Ok(num) => println!("Numeric value 6: {}", num),
        Err(e) => print_error(e),
    }
    match char_to_number('q') {
        Ok(num) => println!("Numeric value q: {}", num),
        Err(e) => print_error(e),
    }

    match char_to_number_hex('A') {
        Ok(num) => println!("Hex value A: {}", num),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('R') {
        Ok(num) => println!("Hex value R: {}", num),
        Err(e) => print_error(e),
    }
}

#[derive(Debug)]
enum ErrType {
    NotFound,
}

fn user_exists(users: &[&str], name: &str) -> bool {
    for &user in users {
        if user == name {
            return true;
        }
    }
    false
}

fn check_user(users: &[&str], name: &str) -> Result<&'static str, ErrType> {
    if user_exists(users, name) {
        Ok("Contul este verificat si se poate conecta.")
    } else {
        Err(ErrType::NotFound)
    }
}

fn err(err: &ErrType) {
    match err {
        ErrType::NotFound => println!("Eroare: Utilizatorul nu a fost gasit."),
    }
}

fn problema5() {
    // aplicatie care imi veridfica daca un anumit utilizator se poate conecta la server, daca apare in lista de users
    let users = ["Ion", "Maria", "Andrei"];
    let check_names = ["Ion", "Ana", "Andrei", "Elena"];

    for &name in &check_names {
        match check_user(&users, name) {
            Ok(msg) => println!("{}: {}", name, msg),
            Err(e) => err(&e),
        }
    }
}

fn main() {
    println!("Output problema2: ");
    problema1();

    println!("-------------------------------------------------------");

    println!("Output problema2: ");
    problema2();

    println!("-------------------------------------------------------");

    println!("Output problema3: ");
    problema3();

    println!("-------------------------------------------------------");

    println!("Output problema4: ");
    problema4();

    println!("-------------------------------------------------------");

    println!("Output problema5: ");
    problema5();
}
