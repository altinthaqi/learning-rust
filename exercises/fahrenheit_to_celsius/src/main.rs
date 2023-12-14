use std::io;

fn main() {
    loop {
        let mut fahrenheit = String::new();
        println!("Fahrenheit: ");

        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("failed to read input");

        let fahrenheit: u32 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let celsius = (fahrenheit - 32) * 5 / 9;

        println!("{}F is {}C", fahrenheit, celsius);
        break;
    }
}
