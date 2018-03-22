use std::io;

fn main() {
    let mut number = String::new();
    println!("Input Fib Seq Index");

    io::stdin().read_line(&mut number)
        .expect("Failed to read line");

    let number: u64 = match number.trim().parse(){
        Ok(num) => num,
        Err(_) => 0,
    };

    let fib_num = fibonacci(number);
    println!("The fib num is: {}", fib_num);

}

fn fibonacci(n: u64) -> u64 {
    let mut x1 = 0;
    let mut x2 = 1;

    if n == 0 {
        return x1;
    } else if n == 1 {
        return x2;
    }

    // now we calculate it
    let mut sum = 0;
    for number in 1..n {
        sum = x1 + x2;
        x1 = x2;
        x2 = sum;
    }
    
    return sum;
}