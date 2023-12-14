use std::io;

//non recursive
fn main() {
    loop {
        let mut series = String::new();

        println!("N series of fibonacci: ");

        io::stdin()
            .read_line(&mut series)
            .expect("failed to read input");

        let series: u32 = match series.trim().parse() {
            Ok(series) => series,
            Err(_) => continue,
        };

        let mut sum = 0;

        if series == 0 {
            panic!("{} is not a right arg for fib", series);
        }
        if series == 1 {
            sum += 1;
        }

        let mut last = 0;
        let mut curr = 1;

        for _i in 1..series {
            sum = last + curr;
            last = curr;
            curr = sum;
        }

        println!("sum {}", sum);
        break;
    }
}
