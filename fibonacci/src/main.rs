fn prime_before_n(n: usize) {
    let mut prime: Vec<usize> =  Vec::new();
    for i in 2..n + 1 { // i = 7
        let mut is_prime = true;

        for div in 2..i { // div
            if i % div == 0 { // i % div
                is_prime = false;
                break;
            }
        }
        if is_prime{
            // i
            prime.push(i);
        }
    }
    println!("{:?}", prime);
}

fn main() {
    let n = 100;

    prime_before_n(n);
}