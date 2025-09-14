fn main() {
    let x = 4;
    println!("value of x is: {x}");

    {
        let x = x * 2;
        println!("value of x in inner scope is: {x}");
    }

    let x = "hello";
    println!("new value of x is: {x}");
}
