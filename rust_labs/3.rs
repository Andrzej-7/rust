fn lcg(seed: &mut u64, multiplier: u64, increment: u64, modulus: u64) -> u64 {
    *seed = (multiplier * (*seed) + increment) % modulus;
    *seed
}

fn main() {
    let mut seed = 123456789;
    let rand_num = lcg(&mut seed, 1, 10, 5);


    println!("{}", rand_num);
    
}
