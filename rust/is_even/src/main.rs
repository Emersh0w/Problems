fn main() {
    let result = is_even(5);
    println!("{result}");
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
} 