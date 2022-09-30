fn main() {
    let N = 5;
    let mut A = [2, 2, 2, 6, 3];
    let B = [1, 2, 3, 4, 5];
    let mut count = 0;
    let mut next_marble = false;

    for marble in 0..=N {
        next_marble = false;
        while next_marble == false {
            if A.len() == marble {
                break;
            }

            if A[marble] < B[marble] {
                A[marble] += 1;
                A[marble + 1] -= 1;
                count += 1;

                println!("{:?}", A);
            }

            if A[marble] > B[marble] {
                A[marble] -= 1;
                A[marble + 1] += 1;
                count += 1;

                println! {"{:?}", A};
            }

            if A[marble] == B[marble] {
                next_marble = true;
            }
        }
    }
    println!("{}", count);
}
