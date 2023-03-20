fn main() {
    let month = 3;
    let year = 2000;

    let days_in_month = if month == 2 && (year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)) {
        29
    } else if month == 2 {
        28
    } else if month == 4 || month == 6 || month == 9 || month == 11 {
        30
    } else {
        31
    };

    println!("There are {} days in month {} of year {}.", days_in_month, month, year);
}
