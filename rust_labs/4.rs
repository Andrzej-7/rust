fn f(x: f64) -> f64 {
    x.powi(2) - 4.0
}

fn fp(x: f64) -> f64 {
    2.0 * x
}

fn met_newt(x0: f64, eps: f64, N: u128) -> f64 {
    let mut x = x0;
    let mut n = 0;
    let mut fx = f(x);

    for i in 0..N+1 { 
        let fp_x = fp(x);

        x = x - fx / fp_x;

        fx = f(x);
        n += 1;

        if fx.abs() < eps {
            return x;
        }
    }

    x
}

fn main() {
    let x0 = 1.0;
    let eps = 0.0000000001;
    let N = 1000;

    let root = met_newt(x0, eps, N);

    println!("Root = {}", root);
}
