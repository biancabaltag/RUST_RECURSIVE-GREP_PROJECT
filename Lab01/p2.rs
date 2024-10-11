fn coprime(mut a: u32, mut b: u32) -> bool {
    let c = a;

    if a == 0 || b == 0 {
        return false;
    }
    
    if a < b {
        a = b;
        b = c;
    }
    
    if a % b == 0 {
        return false;
    }
    let mut i = 2;

    while i <= b / 2 {
        if (a % i == 0) && (b % i == 0) {
            return false;
        }

        i += 1;
    }

    return true;
}

fn main() {
    let mut i = 0;
    let mut j = 0;

    while i <= 100 {
    
        while j <= 100{
            if coprime(i, j) {
                println!("{i},{j}");
            }
            j += 1;
        }
        j = 0;
        i += 1;
    }
}
