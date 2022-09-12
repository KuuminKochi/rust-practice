// Create a flipfront function that takes two values. The first value will be an array, and the second will be a number (n)
// The elements from 1 to nth will be reversed and flipped
//
// Example: list = [2, 5, 7, 4, 3]
//          n = 3
//
//                    |
//          [2, 5, 7, 4, 3]
//          [7, 5, 2, 4, 3]
//           ^  ^  ^ 
//           These values have been reversed and flipped
//
// Problem: Given an array, flip the elements that come before n
// Sub-problem: Iterate through the elements, and push from the nth element to the first element

fn flipfront(array: Vec<usize>, index: usize) {
    
    let arr = &array[0..index];
    let arr2 = &array[index..array.len()];

    let out = [arr.into_iter().rev().collect::<Vec<_>>(), arr2.into_iter().collect::<Vec<_>>()].concat();

    println!("{:?}", out);
}

fn main() {

    let array = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let n = 3;

    // Expected output: 
    flipfront(array, n)
}
