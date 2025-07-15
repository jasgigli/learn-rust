fn main() {
    let mut x: i8 = 5;
    println!("The value of x is : {x}");
    x = 80;
    println!("The new value of x is: {x}");

    constant();
    shadowing();
}

fn constant() {
    const PI_VALUE: u32 = 314;
    println!("The value of PI_VALUE is: {PI_VALUE}");
}
fn shadowing() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The inner scope x is: {x}")
    }

    println!("The outer scope x is: {x}")
}
