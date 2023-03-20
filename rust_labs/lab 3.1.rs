fn swap(&mut x: i32, &mut y: i32) {
    let mut temp = x;
    y = x;
    y = temp;
}

fn main() {
    let mut x: i32 = 4;
    let mut y: i32 = 6;

    println!("{} {}", x, y);
}