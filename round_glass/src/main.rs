fn main() {

    let n = 4;
    let a = 5;
    let mut r = vec![2, 3, 6];
    let mut inc = 1;
    let mut sum = 0;

    r.sort();
    r.reverse();

    for cand in r[n - 2].clone()..=r[0].clone() {
        let mut r_cand = r.clone();
        
        r_cand.push(cand);
        r_cand.sort();
        r_cand.reverse();

        for (i, j) in r_cand.clone().into_iter().enumerate() {
            r_cand[i] = j * j;
        }

        while inc < n {
            r_cand[inc] *= -1;
            inc += 2;
        }
        
        for num in r_cand.clone() {
            sum += num;       
        }

        if sum == a {
            println!("{}", cand);
            break;
        } else {
            inc = 1;
            sum = 0;
        }
    }
}
