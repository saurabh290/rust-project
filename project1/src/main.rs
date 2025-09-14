fn main() {
    let tuple = (4, "saurabh", 7, 9.8);
    println!("Tuple: {}", tuple.1);

    let arr: [i32; 5] = [1,2,3,4,5];
    println!("Array: {}", arr[2]);

    let x = 445900_i64;
    let y = 100 as i32;
    let z = x / (y as i64);
    println!("z: {}", z);
}
