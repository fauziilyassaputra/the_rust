fn main() {
    let _v: Vec<i32> = Vec::new();
    let v = vec![1,2,3];
    println!("the value is : {:?}" ,v);

    // update a vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(15);

    println!("the value v is : {:?}", v);

    // reading element
    let third: &i32 = &v[2];
    println!("The third element is : {}", third);

    let four: Option<&i32> = v.get(3);
    match four {
        Some(four) => println!("The four element is: {four}"),
        None => println!("there is not four element")
    }
    // try to out of range
    let one_hundred: Option<&i32> = v.get(100);
    match one_hundred {
        Some(one_hundred) => println!("The 100 element is: {one_hundred}"),
        None => println!("there is not 100 element")
    }
    // iterator
    let v = vec![10,50,36,42,12];
    for i in &v {
        println!("{i}")
    }
    let mut v = vec![5,10,15];
    for i in &mut v {
        *i += 50; // * = Dereference operator
        println!("{i}")
    }

    // using an enum for different types
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadSheetCell::Int(30),
        SpreadSheetCell::Text(String::from("orange")),
        SpreadSheetCell::Float(3.14159)
    ];

}
