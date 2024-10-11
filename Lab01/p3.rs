fn main() {
    let mut b = 99;

    while b > 0 {
        println!(
            "{} bottle{} of beer on the wall,",
            b,
            if b == 1 { "" } else { "s" }
        );
        println!(
            "{} bottle{} of beer.",
            b,
            if b == 1 { "" } else { "s" }
        );

        b -= 1;

        if b == 0 {
            println!("Take one down, pass it around,");
            println!("No more bottles of beer on the wall!");
        } else {
            println!("Take one down, pass it around,");
            println!(
                "{} bottle{} of beer on the wall!",
                b,
                if b == 1 { "" } else { "s" }
            );
        }

        println!();
    }

    println!("No more bottles of beer on the wall,");
    println!("No more bottles of beer.");
    println!("Go to the store and buy some more,");
    println!("99 bottles of beer on the wall!");
}
