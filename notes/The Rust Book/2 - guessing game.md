rust has a strong static type system
and type inference

to import a library
`use std::io;`

to view crate docs -> `cargo doc --open`


prelude -> default items from the rust's standard library
if it's not a prelude, you `use` it

creating a var
`let mut guess = String::new()`
variables are immutable by default
`::` is an associate function implemented on a type


`io::stdin()`
call the stdin function from io module

`.read_line(&mut guess)
call read_line method on the input handler
pass argument where to store/output the guess

`&` -> a reference, let's multiple part of our code to access one piece of data

`.expect("Failed to read line");
potential failure
Ok || Err
program complies, but throws a warning if we don't `.expect`

printing with a placeholder
`println!("You guessed: {guess}");`

reproducible build with cargo.lock file -> uses only the version of deps specified unless indicated otherwise

update cargo crates -> `cargo update`

ranges -> `start..=end`


you compare values with `match`
a `match` is made up of arms -> a Pattern & a Code to run if it does find a match
rust takes the value given to the match, and looks through patterns in turn
```
match guess.cmp(&secret_number) {
	Ordering::Less => println!("Too small"),
	...
```

a pattern can be consisted also of an `enum` which in the above case is `Ordering`
with variants such as `Less`, `Greated` and `Equal`.
```
match guess.cmp(&secret_number) {
	Ordering::Less => println!("Too small"),
	Ordering::Greater => println!("Too big"),
	Ordering::Equal => println!("You wins"),
}
```


shadowing -> let's us reuse an already declared variable
it's used when we want to change a variable from one type to the other
```
let guess = String::new();
...
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

when you get input from user, on enter key they insert `\n`, to remove this you can `.parse()`
you can also use `parse` to change a variable from a type to another type, with the use of `:`
```
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

in order to loop a code block, you can use `loop` (this introduces an infinite looping system)
```
loop {
	...
}
```
to introduce a stopping to the loop, you need a condition with `break;`


to handle invalid input, use `match`
```
let guess: u32 = match guess.trim().parse() { 
	Ok(num) => num, 
	Err(_) => continue, 
};
```
this is so when we can not correctly pass to a number, the game was crashing beforehand - but now it just restarts the loop

the `match` returns `Ok` and `Err` that you can use. `Ok` contains the resultant value