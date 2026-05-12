fn main() {
    let x = 3.0;
    let y: f32 = 4.0;

    let t = true;
    let f: bool = false;


    let c = 'z';

    // tuple
    let tup: (i32,f64, u8) = (500,6.4,2);
    let (x,y,z) = tup;

    println!("the value of y is: {y}");

    let x: (i32,f64, u8) = (500,6.4,2);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let two = x.2;

    // Array
    let a = [1,2,3,4,5];
    let a =[3;5]; // [3,3,3,3]

    // invalid array element access (Panic!)
    let example = [1,2,3,4,5];
    let index_10 = example[10];
    println!("index 10: {index_10}")


}
