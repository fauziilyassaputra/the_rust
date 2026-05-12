fn main() {

    // variable is imutable by default

    //for example (Error)
    // let x = 5;
    // println!("The value x is: {x}");
    // x = 3;
    // println!("The value x is: {x}");


    // for example (with mutable)
    let mut x = 5;
    println!("The value x is: {x}");
    x = 3;
    println!("The value x is: {x}");


    const THREE_HOURS_IN_SECOND: u32 = 60 * 60 * 3;
    println!("3 hour in second: {THREE_HOURS_IN_SECOND}");

    // shadowing
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The Value of x in the inner scope is: {x}");
    }
    println!("The value y is: {y}");

    // mut vs shadowing

    let spaces = "   ";
    let spaces = spaces.len();

    let mut spaces = "  ";
    spaces = spaces.len() // Error!
}
