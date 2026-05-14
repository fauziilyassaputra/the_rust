fn main() {
let s1 = String::from("Hello world");
let s2 = calculate_function(&s1);
println!("the length of '{s1}' is: {s2}");


// use mut if want change borrowing variable (because it's imutable by default)
let mut s1 = String::from("Hello");
change(&mut s1);

// can't borrow more than once at a time
let mut s1 = String::from("Hello everyone");

let b1 = &mut s1;
// let b2 = &mut s1;
// println!("{b1} {b2}");

}

fn calculate_function(s: &String) -> usize {
    s.len()
}
fn change(string: &mut String)  {
    string.push_str(", world");
}