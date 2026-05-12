fn main() {
    println!("Hello, world!");

    another_function();
    function_params(12);
    let x = five();
    println!(" return value function five() is: {x}");

    // plus_one_error(5);
    plus_one(5);
    plus_one_return(5);
}

fn another_function(){
    println!("another function")
}

fn function_params(x: i32){
    println!("the value x is: {x}")
}

// return values
fn five()-> i32{
    5
}

// fn plus_one_error(x: i32) -> i32{
//     x + 1; // error : this is statement (not return value) , not expression (return value)
// }

fn plus_one(x:i32) -> i32{
    x + 1
}


fn plus_one_return(x:i32) -> i32{
    return x + 1;
}