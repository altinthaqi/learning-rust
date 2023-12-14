use std::io;

fn main() {
    let a: [&str; 3] = ["first", "second", "third"];
    let mut index = String::new();

    println!("Which element would you like to access? ");

    io::stdin().read_line(&mut index).expect("Failed!");

    let index: usize = index.trim().parse().expect("Not a number!");

    let element = a[index];

    println!("This is the element you wanted {}", element);

    let tup: (i32, &str, f64) = (10, "hey", 2.2);
    println!("first element from tuple {}", tup.1);

    let unsigned: i32 = 3123;
    println!("Unsigned {}", unsigned)
}
