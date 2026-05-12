fn main() {
    // If statement return boolean,

    // example if not bool
    // let x = 5;
    //
    // if x { // ERROR !
    //     println!("number was five");
    // }

    // using if in let statement
    let condition =true;
    let number_five = if condition {5} else {5};
    // Example Error:
    // let number_five = if condition {5} else {"six"};


    println!("The value is: {number_five}");


    // while
    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // for
    println!("with for :");
    for element in a {
        println!("The value is: {element}")
    }

    for number in (1..10).rev(){
        println!("{number}")
    }

}
