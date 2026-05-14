fn main() {

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];  

    println!("{hello} {world}");

    let slice = &s[0..2];
    let slice2 = &s[..2];
    println!("{slice} {slice2}");

 
}
