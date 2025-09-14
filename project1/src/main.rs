fn main() {
    let x = 4;
    let CONSTANT_VALUE: u32 = 100_000; // constant value

    println!("constant value is: {CONSTANT_VALUE}");

    println!("value of x is: {x}");

    {
        let x = x * 2;
        println!("value of x in inner scope is: {x}");
    }

    let x = "hello";
    println!("new value of x is: {x}");
}
