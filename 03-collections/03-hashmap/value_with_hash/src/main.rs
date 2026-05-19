
fn main() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 20);
    println!("score is {:?}", scores);

    let team_name = String::from("blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score blue is: {:?}", score);

    // loop
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // ownership in hashmap
    let field_name = String::from("favorite color");
    let field_value = String::from("blue sky");
    let mut map = HashMap::new();
    map.insert(field_name,field_value);
    println!("map is {:?}", map);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("name and value: {:?} {:?}", field_name,field_value) // ERROR

    //Overwriting
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    // or_insert
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Blue")).or_insert(30);
    scores.entry(String::from("Yellow")).or_insert(20);

    println!("{scores:?}"); // Blue: 10

    // update a value based on old value
    let text = "i like rust and you like rust too";

    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}")


}
