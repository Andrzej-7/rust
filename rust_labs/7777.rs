fn swap(x: &mut i32, y: &mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}

fn main() {
    let mut x: i32 = 4;
    let mut y: i32 = 6;
    println!("x={} y={}", x, y);
    
    swap(&mut x, &mut y);

    println!("x={} y={}", x, y);
}
