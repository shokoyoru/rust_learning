use std::io;

fn add(numbers: &[f64]) -> f64 {
    numbers.iter().sum()
}

fn sub(numbers: &[f64]) -> f64 {
    let first = numbers[0];
    numbers[1..].iter().fold(first, |acc, &x| acc-x)
}

fn multi(numbers: &[f64]) -> f64 {
    numbers.iter().product()
}

fn divide(numbers: &[f64]) -> f64 {
    let first = numbers[0];
    numbers[1..].iter().fold(first, |acc, &x| {
        if x != 0.0 {
            acc/x
        }else{
            println!("division cannot divide by zero");
        acc
        }
    })
}

fn pow(numbers: &[f64]) -> f64 {
     let first = numbers[0];
     numbers[1..].iter().fold(first, |acc, &x| acc.powf(x))
}

fn modulus(numbers: &[f64]) -> f64 {
     let first = numbers[0];
     numbers[1..].iter().fold(first, |acc, &x| acc % x)
}

fn sqrt_numbers(numbers: &[f64]) ->Vec<f64> {
    numbers.iter().map(|&x| {
        if x <0.0 {
        println!("error! negative number");
        0.0
        }else{
            x.sqrt()
        }
    }).collect()
}
  
fn get_numbers(min_count: usize) -> Vec<f64> {
    let mut numbers = Vec::new();

    while numbers.len() < min_count {
        let mut input = String::new();
        println!("enter numbers");
        io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    let mut new_numbers: Vec<f64> = input
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse::<f64>().ok())
        .collect();

    numbers.append(&mut new_numbers);

    }
    
    numbers
}

fn main(){

    loop{

    println!("\n---Calculator---\n");
    println!("1-add 2-sub 3-multi 4-divide 5-pow 6-mod 7-sqrt 0-exit\n");

    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");

    let choice: u32 = match input.trim().parse (){ 
        Ok(num) => num,
        Err(_) => {
            println!("invalid input");
            continue;
        }
    };

    if choice == 0{
        println!("program ended");
        break;
    }

    let min_numbers = match choice {
        1 | 3 | 4 | 5 | 6 => 2,
        _=> 1,
    };

    let numbers = get_numbers(min_numbers);
    if numbers.is_empty() {
        println!("unvalid numbers entered");
        continue;
    }

    let result = match choice {
        1 => add(&numbers),
        2 => sub(&numbers),
        3 => multi(&numbers),
        4 => divide(&numbers),
        5 => pow(&numbers),
        6 => modulus(&numbers),
        7 => {
            let roots = sqrt_numbers(&numbers);
            println!("Square roots: {:?}", roots);
            continue;
        }
        _ => {
            println!("invalid choice");
            continue;
        }
    };

        println!("result: {}", result);
    }
}

    
