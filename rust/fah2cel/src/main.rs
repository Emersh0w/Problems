use std::io; 

fn main() {
    println!("\nFarenheit °F and Celsius °C!\n");

    loop {
        println!("Input your value:");

        let mut value: String = String::new();

        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line");

        let mut value: i32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("°F or °C? (Either 'F' or 'C')");

        let mut temp: String = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        if temp.contains("F") {
            value = (value - 32) / (9/5);

        } else if temp.contains("C") {
            value = (value * 9/5) + 32;

        } else {
            println!("°F or °C? (Either 'F' or 'C')");
        }

        println!("Your new value is: {value}");
        break;
    };
}
