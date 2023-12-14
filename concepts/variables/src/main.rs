fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("value inside scope {}", x)
    }
    println!("value outside scope {}", x);

    let spaces = "     ";
    let spaces = spaces.len();

    println!("spaces: {}", spaces);

    //needs shadowing, can't mut
    // let mut mut_spaces = "     ";
    // mut_spaces = mut_spaces.len();
    // println!("mutSpaces: {}", mut_spaces)
}
