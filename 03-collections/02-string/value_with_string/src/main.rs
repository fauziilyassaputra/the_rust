fn main() {

    // create a new string
    let mut _s = String::from("");

    let _data = "initial contents";
    let _s = _data.to_string();
    let _s = "initial content".to_string();
    let _s = String::from("initial contents");

    // updating a string
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("the value of s is: {s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is: {s1}");
    println!("s2 is: {s2}");

    // push with single string
    let mut s = String::from("lo");
    s.push('l');
    println!("s is: {s}");

    // with + format
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is: {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is: {s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("s is: {s}");

    // reading a bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }


}
