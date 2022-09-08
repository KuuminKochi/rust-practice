// 1. Assign a number with each letter of the alphabet
//          - a : 1
//          - b : 2
//          - c : 3
// Problem: How to assign a number to each letter of the alphabet
// Solution: Use a hashmap, make the keys each letter of the alphabet (Has to be done manually, will use a vector for this) 
//           and the value each number from 1 to 26 (a to z)
//
// 2. Split the string into seperate chars
// Problem: How to split a string into separate chars?
// Solution: Use .char() (Convert it into ascii.lowercase as well)
//
// 3. Check what the letter given is which number in the hashmap
// Problem: How do I check which letter is associated with each number?
// Solution: Use a for loop to iterate all the chars in a given string (which should be in a vector of chars by now)
//              
// 4. I already finished the code im too lazy to write anything down


use std::collections::HashMap;

fn letter_value_sum (word: String) {
    let mut sum = 0;
    let mut sum_list = vec![];
    let word_vec = word.to_ascii_lowercase()
                                    .chars()
                                    .filter(|&x| x != ' ')
                                    .collect::<Vec<_>>();
    let mut count = 0;
    let mut number_list = vec![];
    let letters_list = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let mut letter_val: HashMap<char, i32> = HashMap::new();

    for i in 1..=27 {
        number_list.push(i);
    }

    while count < 26 {
        letter_val.insert(letters_list[count], number_list[count]);
        count += 1;
    }

    for letter in word_vec {
        sum_list.push(letter_val[&letter]);
    }

    for i in sum_list {
        sum += i;
    }

    println!("{}", sum);

}

fn main () {
    let word : String = "a man has fallen into a river in lego city".to_string();

    letter_value_sum(word);
}