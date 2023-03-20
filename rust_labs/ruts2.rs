fn swap_arr(arr: &mut [i32], i: usize, j: usize) {
    let tmp = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp;
}


fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    swap_arr(&mut arr, 1, 3);
    println!("{:?}", arr); //zeby napisac w takim formacie [1, 4, 3, 2, 5]
}
