fn main() {
    let mut count = 0;
    let Q = 4;
    let H = [3, 2, 7, 36];
    let W = [5, 2, 2, 28];
    let T = [5, 5, 0, 127];
    let mut x = 1;
    let mut y = 1;

    let mut not_first = false;

    let mut x_inc = 1;
    let mut y_inc = 1;

    for test in 0..=Q - 1 {
        while count < T[test] {
            if x == H[test] || (x == 1 && not_first) {
                x_inc = x_inc * -1
            }

            if y == W[test] || (y == 1 && not_first) {
                y_inc = y_inc * -1
            }

            x += x_inc;
            y += y_inc;

            count += 1;
            not_first = true;
        }

        println!("{} {}", x, y);

        x = 1;
        y = 1;
        x_inc = 1;
        y_inc = 1;

        count = 0;
        not_first = false;
    }
}
