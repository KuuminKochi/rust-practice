// Fibonacci
// Add the numbers after a number before the prior number
// 1, 1, 2, 3, 5, 8, 13

fn fibonacci(index: usize) {
    let mut number = vec![0, 1];
  
    for count in 0..index - 1 { 
        let sum = number[0 + count] + number[1 + count];
        number.push(sum);
    }
    println!("{:?}", number);
}
fn main() {
    fibonacci(7);
}
