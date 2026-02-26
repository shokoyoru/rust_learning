use std::io::*;

fn main() {
    let mut input = String::new();

    loop {
        input.clear();
        println!("Enter first number | X|x for exit: ");
        stdin().read_line(&mut input).unwrap();
        let exit = input.as_str().trim().to_lowercase();
        if exit == "x" {
            break;
        } else {
            let x = input.as_str().trim().parse::<i64>().unwrap();

            input.clear();

            println!("Enter second number | X|x for exit: ");
            stdin().read_line(&mut input).unwrap();
            let exit = input.as_str().trim().to_lowercase();
            if exit == "x" {
                break;
            } else {
                let y = input.as_str().trim().parse::<i64>().unwrap();

                let result = x + y;

                println!("{} + {} = {}", x, y, result);
            }
        }
    }
}