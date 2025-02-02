use std::collections::HashMap;
use std::io;
use num_bigint::BigUint;
use num_traits::{Zero, One};

fn fibonacci(n: u32, memo: &mut HashMap<u32, BigUint>) -> BigUint {
    if n == 0 {
        return Zero::zero();
    } else if n == 1 {
        return One::one();
    }

    if let Some(value) = memo.get(&n) {
        return value.clone();
    }

    let result = fibonacci(n - 1, memo) + fibonacci(n - 2, memo);
    memo.insert(n, result.clone());
    result
}

fn main() {
    let mut input = String::new();
    println!("Enter the number of Fibonacci terms:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: u32 = input.trim().parse().expect("Please enter a valid number");

    let mut memo = HashMap::new();
    for i in 0..n {
        let fib_number = fibonacci(i, &mut memo);
        let length = fib_number.to_string().len();
        println!("Fibonacci number {}:  ({} digits long)", i, length);
    }
}