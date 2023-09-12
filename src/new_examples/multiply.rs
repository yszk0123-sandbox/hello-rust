use std::num;

type MultiplyResult = Result<i32, num::ParseIntError>;

fn multiply_naive(first_number_str: &str, second_number_str: &str) -> MultiplyResult {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => match second_number_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn multiply(first_number_str: &str, second_number_str: &str) -> MultiplyResult {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn multiply_early_return(first_number_str: &str, second_number_str: &str) -> MultiplyResult {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;
    Ok(first_number * second_number)
}

fn print(result: MultiplyResult) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply(&"100", &"200"));
}
