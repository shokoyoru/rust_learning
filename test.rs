use std::io;

fn main(){
    let mut input = String::new();

    println!("enter first number: ");
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");
    let num1: f64 = input.trim().parse().expect("invalid value");

    input.clear();

    println!("enter second number: ");
    io::stdin().
    read_line(&mut input)
    .expect("Failed to read input");
    let num2: f64 = input.trim().parse().expect("invalid value");

    input.clear();

    println("plus: {}", num1 + num2);
    println("subtract: {}", num1 - num2);
    println("multi: {}", num1 * num2);
    
    if num2 != 0.0 {
        println("division: {}", num1 / num2)
    }else {
        println("division cannot divide by zero")
    }
}