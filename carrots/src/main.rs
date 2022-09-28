fn main() {
    // Test cases
    let t = 3;
    // Number of baskets
    let n = [3, 4, 5];
    // Number of carrots in each baskets
    let c = [vec![3, 5, 2], vec![4, 6, 2, 3], vec![4, 8, 10, 5, 2]];

    // Make sure that it'll only print for each test case

    for count in 0..t {
        let mut even_count = 0;
        let mut odd_count = 0;
        for num in c[count].clone() {
            if num % 2 == 1{
                odd_count += 1;
            } else {
                even_count += 1;
            }
        }
            

        if even_count >= 2 && odd_count >= 1 {
            println!("YES");
        } else if odd_count % 2 == 1 {
            println!("YES");
        } else {
            println!("NO");
        }
    } 

//    while count < t {
//        count += 1;
//    }
}
