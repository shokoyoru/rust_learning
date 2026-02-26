use std::io::stdin;

fn main()  {
    let mut input = String::new();

    println!("Enter first number:");
    stdin().read_line(&mut input).unwrap();
    let x = input.as_str().trim().parse::<i64>().unwrap();

    input.clear();

    println!("Enter second number:");
    stdin().read_line(&mut input).unwrap();
    let y = input.as_str().trim().parse::<i64>().unwrap();

    let result = x + y;
    println!("{} + {} = {}", x, y, result);
}
