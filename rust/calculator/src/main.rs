use std::{env::{Args, args}};

fn main() {
    let mut args:Args = args();

    let first: String = args.nth(1).unwrap();
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second: String = args.nth(0).unwrap();

    let first_number: f32 = first.parse::<f32>().unwrap();
    let second_number: f32 = second.parse::<f32>().unwrap();

    let result: f32 = calculate(operator, first_number, second_number);

    output(first_number, operator, second_number, result);

    println!("{result}");
}   


fn calculate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Invalid Operator used! --> '{operator}' ")
    }
}


fn output (first_number: f32, operator: char ,second_number: f32, result: f32) -> String {
    format!("{first_number} {operator} {second_number} = {result}")
}