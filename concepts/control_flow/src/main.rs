use std::io;

fn main() {
    conditionals();

    loops();

    innner_loops();

    while_loop();

    collection_looping();
}

fn collection_looping() {
    let a: [i32; 3] = [1, 2, 3];

    let mut index = 0;
    while index < a.len() {
        println!("value: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("element: {}", element)
    }
}

fn while_loop() {
    let mut number = 5;

    while number != 0 {
        println!("number: {}", number);
        number -= 1
    }
}

fn innner_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);

        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1
        }

        count += 1;
    }

    println!("final count: {}", count)
}

fn loops() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        } else {
            println!("iteration nr. {}", counter);
        }
    };
    println!("counter results {}", result);
}

fn conditionals() {
    let number: i8 = 32;
    if number != 8 {
        println!("Number is diff")
    }

    let assigned_value_from_expression: i32 = if number != 32 { 5 } else { 8 };
    println!("{}", assigned_value_from_expression);
}
