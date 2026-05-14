fn main() {
    // owner ship rules

    //1. variable scope
    let s = "hello";
    {
        let s = "hello";
        println!("s value is: {s}")
    } // drop let s in scope


    //2. the string type
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    // variables and data interacting with move
    let x = 5;
    let y = x;
    let s1 = String::from("hello");
    // let s2 = s1;

    println!("data s1 : {s1}"); // Error s1 is not valid (Not double free error)
    
    
    // clone (only heap data)
    let s2 = s1.clone();
    println!("data s1 : {s1}"); // Error s1 is not valid (Not double free error)


    // owner ship and function
    let x = 30;
    takes_ownership(x);
    println!("value is (not fn): {x}");


    let some_string = String::from("Hello world");
    take_some_string(some_string);
    // println!("value string not fn: {some_string}") // error 

    // RETURN value 

    let s1 = String::from("welcome here");
    let (s2, len) = calculate_length(s1);
    println!("the length of s2: {len}")

}


fn takes_ownership(x: i32 ){
    println!("value is: {x}");
}

fn take_some_string(some_string: String){
    println!("value string is: {some_string}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}