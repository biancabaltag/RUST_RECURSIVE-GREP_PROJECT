fn prim(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i < n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    return true;
}

fn main() {
    let mut i = 2;
    while i <= 100 {
        if prim(i) == true {
            println!("{i}");
        }

        i = i + 1;
    }
}
