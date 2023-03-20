fn main() {

    let liczba = 15;
    let mut silnia: u64 = 1;

    for i in 1..=liczba {
        silnia *= i;
    }

    println!("{}", silnia);
}
