fn main() {
    println!("Hello, world!");
    let v: Vec<i32> = Vec::new();

    let mut c = Vec::new();
    c.push(5);
    c.push(6);
    c.push(7);
    c.push(8);

    let d = vec![1,2,3,4,5];

    let third: &i32 = &d[2];
    println!("The third element is {third}");

    let third: Option<&i32> = d.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There's no third element!"),
    }
}

enum Spread {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec! [
    Spread::Int(3),
    Spread::Text(String::from("blue")),
    Spread::Float(10.12),
]