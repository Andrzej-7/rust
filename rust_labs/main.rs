//use std::io;

fn main() {
    let rok = 1999;
    //io::stdin().read_line(&mut rok).expect("failed");

    if rok % 4 == 0 || rok % 100 == 0 && rok % 400 == 0 {
        println!("good {}", rok);
    }
    else {
         println!("bad {}", rok);
        }



    

    //println!("{}", rok);

        

    

    


}