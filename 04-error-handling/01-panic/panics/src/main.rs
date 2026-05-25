use std::fs::File;
use std::io::{self, Read};
fn main() {
    // panic!("crash and burn");
    // run with : RUST_BACKTRACE=1 cargo run

    // Error with result

    // let greeting_file_result = File::open("hello.txt");
    //
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {error:?}"),
    // };

    // shortcut for panic and errors
    // let greeting_file = File::open("hello.txt").unwrap();
    // let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");

    let result = read_username_from_file(String::from("hello.txt"));

    match result {
        Ok(name) => {
            println!("success!, username in this file is : {}", name)
        }
        Err(error) => {
            println!("cannot read file, err : {:?}", error)
        }
    }
    let result = read_username_from_file_shortcut(String::from("hello.txt"));

    match result {
        Ok(name) => {
            println!("success!, username in this file is : {}", name)
        }
        Err(error) => {
            println!("cannot read file, err : {:?}", error)
        }
    }
    // example ? for option

    let text1 = "welcome to rust\n this is...";
    let result_text1 = last_char_of_first_line(text1);

    let text2 = "";
    let result_text2 = last_char_of_first_line(text2);

    let text3 = "\n this is a good programming language";
    let result_text3 = last_char_of_first_line(text3);

    println!("result text1: {:?}", result_text1);
    println!("result text2: {:?}", result_text2);
    println!("result text3: {:?}", result_text3);



}

fn read_username_from_file(file_name: String) -> Result<String, io::Error>{
    let username_file_result = File::open(file_name);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }


}
// operator shortcut

fn read_username_from_file_shortcut(file_name: String) -> Result<String, io::Error>{
    // let mut username_file = File::open(file_name)?;
    // let mut username = String::new();
    // username_file?.read_to_string(&mut username)?;
    // Ok(username)

    let mut username = String::new();
    File::open(file_name)?.read_to_string(&mut username)?;
    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char>{
    text.lines().next()?.chars().last()
}