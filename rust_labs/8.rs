fn main(){
    let liczba = 10;
    let mut result:u64 = 0;


    for i in 0..liczba+1{
        result+=i;

    }

    println!("{}", result);
}