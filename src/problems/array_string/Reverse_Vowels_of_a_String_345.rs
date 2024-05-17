pub fn reverse_vowels(s: String) -> String {
    // pointer for left and right chars
    // init with first and last indices
    let mut left = 0;
    let mut right = s.len() - 1;

    // convert s into a vector for easy index reading and mutations
    let mut chars: Vec<char> = s.chars().collect();

    // if pointer cross over, chars will be swapped back in their original place again!
    while left < right {
        // if left pointer char is not a vowel, pump left pointer and move onto next loop iteration
        if !matches!(
            chars[left],
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U'
        ) {
            left += 1;
            continue;
        }

        // if right pointer char is not a vowel, pump right pointer and move onto next loop iteration
        if !matches!(
            chars[right],
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U'
        ) {
            right -= 1;
            continue;
        }

        // else both chars are vowels ans we can swap, and update pointer
        chars.swap(left, right);
        left += 1;
        right -= 1;
    }

    // convert back into string
    chars.iter().collect()
}

// pub fn reverse_vowels(s: String) -> String {
//     let mut original_chars = Vec::from(s.chars().collect::<Vec<char>>());

//     let mut vowels: Vec<char> = Vec::new();
//     for i in 0..original_chars.len() {
//         match original_chars[i] {
//             'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
//                 vowels.push(original_chars[i]);
//                 original_chars[i] = '_'
//             }
//             _ => (),
//         }
//     }

//     for i in 0..original_chars.len() {
//         if original_chars[i] == '_' && vowels.len() > 0 {
//             let popped = vowels.pop().unwrap();
//             original_chars[i] = popped;
//         }
//     }

//     let out = original_chars.iter().collect();

//     return out;
// }
