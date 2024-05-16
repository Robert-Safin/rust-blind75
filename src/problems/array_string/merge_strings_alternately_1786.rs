pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut build_string = String::new();

    let mut iter1 = word1.chars();
    let mut iter2 = word2.chars();

    loop {
        match (iter1.next(), iter2.next()) {
            (Some(c1), Some(c2)) => {
                build_string.push(c1);
                build_string.push(c2);
            }
            (None, Some(c2)) => {
                build_string.push(c2);
            }
            (Some(c1), None) => {
                build_string.push(c1);
            }
            (None, None) => break,
        }
    }

    build_string
}

// CALLING CHARS.NTH CREATES AN ITERATOR EVERY TIME TO REACH NTH CHAR
// pub fn merge_alternately(word1: String, word2: String) -> String {
//   let mut build_string = String::new();

//   let mut p1 = 0;
//   let mut p2 = 0;

//   while p1 < word1.len() && p2 < word2.len() {
//       build_string.push(word1.chars().nth(p1).unwrap());
//       build_string.push(word2.chars().nth(p2).unwrap());

//       p1 += 1;
//       p2 += 1;

//       if p1 == word1.len() {
//           for i in p2..word2.len() {
//               build_string.push(word2.chars().nth(i).unwrap())
//           }
//       }

//       if p2 == word2.len() {
//           for i in p1..word1.len() {
//               build_string.push(word1.chars().nth(i).unwrap())
//           }
//       }
//   }
//   println!("{build_string}");
//   return build_string;
// }
