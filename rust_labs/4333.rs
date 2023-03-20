fn func(x: &mut i32, y: &mut i32, z: &mut i32) {
    while *x > *y && *y > *z {
        if *x > *y {
            let temp = *x;
            *x = *y;
            *y = temp;
        }

        if *y > *z {
            let temp = *y;
            *y = *z;
            *z = temp; 
        }
    }
}

fn main() {
    let mut x: i32 = 8;
    let mut y: i32 = 4;
    let mut z: i32 = 2;

    func(&mut x, &mut y, &mut z);

    println!("{} {} {}", x, y, z);
}
