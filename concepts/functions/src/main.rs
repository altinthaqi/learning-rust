fn main() {
    another_function("lol", "hey");

    let y = {
        let x = 3;
        x + 1
    };

    println!("value of y is {}", y);

    let unsigned_returned_value = return_value();
    println!("returned value from fn {}", unsigned_returned_value);
}

fn return_value() -> u32 {
    3
}

fn another_function(arg_1: &str, arg_2: &str) {
    println!("Hai from another_function {} {}", arg_1, arg_2)
}
